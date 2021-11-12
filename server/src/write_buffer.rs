use std::future::Future;
use std::sync::Arc;
use std::time::{Duration, Instant};

use futures::future::{BoxFuture, Shared};
use futures::stream::{BoxStream, FuturesUnordered};
use futures::{FutureExt, StreamExt, TryFutureExt};
use tokio::task::JoinError;
use tokio_util::sync::CancellationToken;

use dml::DmlOperation;
use observability_deps::tracing::{debug, error, info, warn};
use trace::span::SpanRecorder;
use write_buffer::core::{FetchHighWatermark, WriteBufferError, WriteBufferReading};

use crate::Db;

use self::metrics::{SequencerMetrics, WriteBufferIngestMetrics};

mod metrics;

/// A `WriteBufferConsumer` is created from a `Db` and a `WriteBufferReading` and
/// sinks records from the inbound streams into the `Db`
#[derive(Debug)]
pub struct WriteBufferConsumer {
    /// Future that resolves when the background worker exits
    join: Shared<BoxFuture<'static, Result<(), Arc<JoinError>>>>,

    /// A token that is used to trigger shutdown of the background worker
    shutdown: CancellationToken,
}

impl WriteBufferConsumer {
    pub fn new(
        mut write_buffer: Box<dyn WriteBufferReading>,
        db: Arc<Db>,
        registry: &metric::Registry,
    ) -> Self {
        let shutdown = CancellationToken::new();

        let ingest_metrics = WriteBufferIngestMetrics::new(registry, &db.rules().name);

        let shutdown_captured = shutdown.clone();
        let join = tokio::spawn(async move {
            let mut futures: FuturesUnordered<_> = write_buffer
                .streams()
                .into_iter()
                .map(|(sequencer_id, stream)| {
                    let metrics = ingest_metrics.new_sequencer_metrics(sequencer_id);
                    stream_in_sequenced_entries(
                        Arc::clone(&db),
                        sequencer_id,
                        stream.stream,
                        stream.fetch_high_watermark,
                        metrics,
                    )
                })
                .collect();

            tokio::select! {
                _ = shutdown_captured.cancelled() => info!("write buffer shut down triggered"),
                _ = futures.next() => error!("unexpected shutdown of write buffer consumer"),
            }
        })
        .map_err(Arc::new)
        .boxed()
        .shared();

        Self { join, shutdown }
    }

    /// Triggers shutdown of this `WriteBufferConsumer`
    pub fn shutdown(&self) {
        self.shutdown.cancel()
    }

    /// Waits for the background worker of this `Database` to exit
    pub fn join(&self) -> impl Future<Output = Result<(), Arc<JoinError>>> {
        self.join.clone()
    }
}

impl Drop for WriteBufferConsumer {
    fn drop(&mut self) {
        if !self.shutdown.is_cancelled() {
            warn!("write buffer consumer dropped without calling shutdown()");
            self.shutdown.cancel();
        }

        if self.join.clone().now_or_never().is_none() {
            warn!("write buffer consumer dropped without waiting for worker termination");
        }
    }
}

/// This is used to take entries from a `Stream` and put them in the
/// mutable buffer, such as streaming entries from a write buffer.
///
/// Note all errors reading / parsing / writing entries from the write
/// buffer are ignored.
async fn stream_in_sequenced_entries<'a>(
    db: Arc<Db>,
    sequencer_id: u32,
    mut stream: BoxStream<'a, Result<DmlOperation, WriteBufferError>>,
    f_mark: FetchHighWatermark<'a>,
    mut metrics: SequencerMetrics,
) {
    let db_name = db.rules().name.to_string();
    let mut watermark_last_updated: Option<Instant> = None;
    let mut watermark = 0_u64;

    while let Some(db_write_result) = stream.next().await {
        // maybe update sequencer watermark
        // We are not updating this watermark every round because asking the sequencer for that watermark can be
        // quite expensive.
        let now = Instant::now();
        if watermark_last_updated
            .map(|ts| now.duration_since(ts) > Duration::from_secs(10))
            .unwrap_or(true)
        {
            match f_mark().await {
                Ok(w) => {
                    watermark = w;
                }
                // skip over invalid data in the write buffer so recovery can succeed
                Err(e) => {
                    debug!(
                        %e,
                        %db_name,
                        sequencer_id,
                        "Error while reading sequencer watermark",
                    )
                }
            }
            watermark_last_updated = Some(now);
        }

        let ingest_recorder = metrics.recorder(watermark);

        // get entry from sequencer
        let dml_operation = match db_write_result {
            Ok(db_write) => db_write,
            // skip over invalid data in the write buffer so recovery can succeed
            Err(e) => {
                warn!(
                    %e,
                    %db_name,
                    sequencer_id,
                    "Error converting write buffer data to SequencedEntry",
                );
                continue;
            }
        };

        let ingest_recorder = ingest_recorder.operation(&dml_operation);

        // store entry
        let mut logged_hard_limit = false;
        loop {
            let mut span_recorder = SpanRecorder::new(
                dml_operation
                    .meta()
                    .span_context()
                    .map(|parent| parent.child("IOx write buffer")),
            );

            let result = match &dml_operation {
                DmlOperation::Write(db_write) => db.store_write(db_write),
            };

            match result {
                Ok(_) => {
                    ingest_recorder.success();
                    span_recorder.ok("stored write");

                    break;
                }
                Err(crate::db::Error::HardLimitReached {}) => {
                    // wait a bit and retry
                    if !logged_hard_limit {
                        info!(
                            %db_name,
                            sequencer_id,
                            "Hard limit reached while reading from write buffer, waiting for compaction to catch up",
                        );
                        logged_hard_limit = true;
                    }
                    span_recorder.error("hard limit reached");

                    tokio::time::sleep(Duration::from_millis(100)).await;
                    continue;
                }
                Err(e) => {
                    // skip over invalid data in the write buffer so recovery can succeed
                    debug!(
                        %e,
                        %db_name,
                        sequencer_id,
                        "Error storing SequencedEntry from write buffer in database"
                    );
                    span_recorder.error("cannot store write");

                    // no retry
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::convert::TryFrom;
    use std::num::NonZeroU32;

    use arrow_util::assert_batches_eq;
    use data_types::sequence::Sequence;
    use persistence_windows::min_max_sequence::MinMaxSequence;
    use query::exec::ExecutionContextProvider;
    use query::frontend::sql::SqlQueryPlanner;
    use write_buffer::mock::{MockBufferForReading, MockBufferSharedState};

    use crate::db::test_helpers::run_query;
    use crate::utils::TestDb;

    use super::*;
    use dml::{DmlMeta, DmlWrite};
    use metric::{Attributes, Metric, U64Counter, U64Gauge};
    use mutable_batch_lp::lines_to_batches;
    use time::Time;

    #[tokio::test]
    async fn read_from_write_buffer_updates_persistence_windows() {
        let partition_key = "1970-01-01T00";

        let write_buffer_state =
            MockBufferSharedState::empty_with_n_sequencers(NonZeroU32::try_from(2).unwrap());

        let sequences = [
            Sequence::new(0, 0),
            Sequence::new(1, 0),
            Sequence::new(1, 2),
            Sequence::new(0, 1),
        ];

        for sequence in sequences {
            write_buffer_state.push_lp(sequence, "cpu bar=1 10");
        }

        let db = TestDb::builder().build().await.db;

        // do: start background task loop
        let shutdown: CancellationToken = Default::default();
        let shutdown_captured = shutdown.clone();
        let db_captured = Arc::clone(&db);
        let join_handle =
            tokio::spawn(async move { db_captured.background_worker(shutdown_captured).await });

        let consumer = WriteBufferConsumer::new(
            Box::new(MockBufferForReading::new(write_buffer_state, None).unwrap()),
            Arc::clone(&db),
            &Default::default(),
        );

        // check: after a while the persistence windows should have the expected data
        let t_0 = Instant::now();
        let min_unpersisted = loop {
            if let Ok(partition) = db.partition("cpu", partition_key) {
                let partition = partition.write();
                let windows = partition.persistence_windows().unwrap();
                let min_unpersisted = windows.minimum_unpersisted_sequence();

                if let Some(min_unpersisted) = min_unpersisted {
                    break min_unpersisted;
                }
            }

            assert!(t_0.elapsed() < Duration::from_secs(10));
            tokio::time::sleep(Duration::from_millis(100)).await;
        };

        // do: stop background task loop
        shutdown.cancel();
        join_handle.await.unwrap();

        consumer.shutdown();
        consumer.join().await.unwrap();

        let mut expected_unpersisted = BTreeMap::new();
        expected_unpersisted.insert(0, MinMaxSequence::new(0, 1));
        expected_unpersisted.insert(1, MinMaxSequence::new(0, 2));

        assert_eq!(min_unpersisted, expected_unpersisted);
    }

    #[tokio::test]
    async fn read_from_write_buffer_write_to_mutable_buffer() {
        let write_buffer_state =
            MockBufferSharedState::empty_with_n_sequencers(NonZeroU32::try_from(1).unwrap());
        let ingest_ts1 = Time::from_timestamp_millis(42);
        let ingest_ts2 = Time::from_timestamp_millis(1337);
        write_buffer_state.push_write(DmlWrite::new(
            lines_to_batches("mem foo=1 10", 0).unwrap(),
            DmlMeta::sequenced(Sequence::new(0, 0), ingest_ts1, None, 50),
        ));
        write_buffer_state.push_write(DmlWrite::new(
            lines_to_batches("cpu bar=2 20\ncpu bar=3 30", 0).unwrap(),
            DmlMeta::sequenced(Sequence::new(0, 7), ingest_ts2, None, 150),
        ));
        let test_db = TestDb::builder().build().await;
        let db = test_db.db;

        // do: start background task loop
        let shutdown: CancellationToken = Default::default();
        let shutdown_captured = shutdown.clone();
        let db_captured = Arc::clone(&db);
        let join_handle =
            tokio::spawn(async move { db_captured.background_worker(shutdown_captured).await });

        let consumer = WriteBufferConsumer::new(
            Box::new(MockBufferForReading::new(write_buffer_state, None).unwrap()),
            Arc::clone(&db),
            test_db.metric_registry.as_ref(),
        );

        let query = "select * from cpu";

        // check: after a while the table should exist and a query plan should succeed
        let t_0 = Instant::now();
        loop {
            let planner = SqlQueryPlanner::default();
            let ctx = db.new_query_context(None);
            let physical_plan = planner.query(query, &ctx).await;

            if physical_plan.is_ok() {
                break;
            }

            assert!(t_0.elapsed() < Duration::from_secs(10));
            tokio::time::sleep(Duration::from_millis(100)).await;
        }

        // do: stop background task loop
        shutdown.cancel();
        join_handle.await.unwrap();

        consumer.shutdown();
        consumer.join().await.unwrap();

        let metrics = test_db.metric_registry;
        let observation = metrics
            .get_instrument::<Metric<U64Counter>>("write_buffer_ingest_requests")
            .unwrap()
            .get_observer(&Attributes::from(&[
                ("db_name", "placeholder"),
                ("sequencer_id", "0"),
                ("status", "ok"),
            ]))
            .unwrap()
            .fetch();
        assert_eq!(observation, 2);

        let observation = metrics
            .get_instrument::<Metric<U64Counter>>("write_buffer_read_bytes")
            .unwrap()
            .get_observer(&Attributes::from(&[
                ("db_name", "placeholder"),
                ("sequencer_id", "0"),
            ]))
            .unwrap()
            .fetch();
        assert_eq!(observation, 200);

        let observation = metrics
            .get_instrument::<Metric<U64Gauge>>("write_buffer_last_sequence_number")
            .unwrap()
            .get_observer(&Attributes::from(&[
                ("db_name", "placeholder"),
                ("sequencer_id", "0"),
            ]))
            .unwrap()
            .fetch();
        assert_eq!(observation, 7);

        let observation = metrics
            .get_instrument::<Metric<U64Gauge>>("write_buffer_sequence_number_lag")
            .unwrap()
            .get_observer(&Attributes::from(&[
                ("db_name", "placeholder"),
                ("sequencer_id", "0"),
            ]))
            .unwrap()
            .fetch();
        assert_eq!(observation, 0);

        let observation = metrics
            .get_instrument::<Metric<U64Gauge>>("write_buffer_last_min_ts")
            .unwrap()
            .get_observer(&Attributes::from(&[
                ("db_name", "placeholder"),
                ("sequencer_id", "0"),
            ]))
            .unwrap()
            .fetch();
        assert_eq!(observation, 20);

        let observation = metrics
            .get_instrument::<Metric<U64Gauge>>("write_buffer_last_max_ts")
            .unwrap()
            .get_observer(&Attributes::from(&[
                ("db_name", "placeholder"),
                ("sequencer_id", "0"),
            ]))
            .unwrap()
            .fetch();
        assert_eq!(observation, 30);

        let observation = metrics
            .get_instrument::<Metric<U64Gauge>>("write_buffer_last_ingest_ts")
            .unwrap()
            .get_observer(&Attributes::from(&[
                ("db_name", "placeholder"),
                ("sequencer_id", "0"),
            ]))
            .unwrap()
            .fetch();
        assert_eq!(observation, ingest_ts2.timestamp_nanos() as u64);

        // check: the expected results should be there
        let batches = run_query(db, "select * from cpu order by time").await;

        let expected = vec![
            "+-----+--------------------------------+",
            "| bar | time                           |",
            "+-----+--------------------------------+",
            "| 2   | 1970-01-01T00:00:00.000000020Z |",
            "| 3   | 1970-01-01T00:00:00.000000030Z |",
            "+-----+--------------------------------+",
        ];
        assert_batches_eq!(expected, &batches);
    }

    #[tokio::test]
    async fn error_converting_data_from_write_buffer_to_sequenced_entry_is_reported() {
        let write_buffer_state =
            MockBufferSharedState::empty_with_n_sequencers(NonZeroU32::try_from(1).unwrap());
        write_buffer_state.push_error(
            String::from("Something bad happened on the way to creating a SequencedEntry").into(),
            0,
        );
        let test_db = TestDb::builder().build().await;

        let db = Arc::new(test_db.db);
        let metric_registry = test_db.metric_registry;

        // do: start background task loop
        let shutdown: CancellationToken = Default::default();
        let shutdown_captured = shutdown.clone();
        let db_captured = Arc::clone(&db);
        let join_handle =
            tokio::spawn(async move { db_captured.background_worker(shutdown_captured).await });

        let consumer = WriteBufferConsumer::new(
            Box::new(MockBufferForReading::new(write_buffer_state, None).unwrap()),
            Arc::clone(&db),
            metric_registry.as_ref(),
        );

        // check: after a while the error should be reported in the database's metrics
        let t_0 = Instant::now();
        let attributes = Attributes::from(&[
            ("db_name", "placeholder"),
            ("sequencer_id", "0"),
            ("status", "client_error"),
        ]);
        loop {
            let maybe_metric = metric_registry
                .get_instrument::<Metric<U64Counter>>("write_buffer_ingest_requests");

            if let Some(metric) = maybe_metric {
                if let Some(observer) = metric.get_observer(&attributes) {
                    if observer.fetch() == 1 {
                        break;
                    }
                }
            }

            assert!(t_0.elapsed() < Duration::from_secs(10));
            tokio::time::sleep(Duration::from_millis(100)).await;
        }

        // do: stop background task loop
        shutdown.cancel();
        join_handle.await.unwrap();

        consumer.shutdown();
        consumer.join().await.unwrap();
    }
}
