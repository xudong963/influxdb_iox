use crate::batch::{BatchStreamExt, Partitioned};
use crate::kinesis::batch::RecordBatcher;
use crate::kinesis::entry::EntryReducer;
use crate::kinesis::limiter::RecordLimiter;
use crate::kinesis::sink::{ErrorHandler, KinesisSink};
use crate::kinesis::topology::{ShardId, TopologyGeneration, TopologyService};
use crate::limiter::LimitedStreamExt;
use futures::future::BoxFuture;
use futures::stream::FuturesUnordered;
use futures::{StreamExt, TryStreamExt};
use influxdb_line_protocol::OwnedParsedLine;
use snafu::Snafu;
use std::time::Duration;
use tokio::sync::{mpsc, oneshot};
use tokio_stream::wrappers::ReceiverStream;
use tokio_util::sync::CancellationToken;

/// A LineRecord is the un-aggregated representation of a single line protocol write
#[derive(Debug)]
pub struct LineRecord {
    pub hash_key: u128,
    pub predicted_shard_id: Option<(ShardId, TopologyGeneration)>,
    pub acker: Option<oneshot::Sender<Result<(), Error>>>,
    pub line: OwnedParsedLine,
}

impl LineRecord {
    pub fn new(line: OwnedParsedLine) -> (Self, oneshot::Receiver<Result<(), Error>>) {
        let (otx, orx) = oneshot::channel::<_>();
        (
            Self {
                hash_key: 0, // TODO: Compute this
                acker: Some(otx),
                predicted_shard_id: None,
                line,
            },
            orx,
        )
    }

    pub fn ack(mut self, res: Result<(), Error>) {
        if let Some(ack) = self.acker.take() {
            // Not an error if upstream has hung up
            let _ = ack.send(res);
        }
    }
}

impl Partitioned for LineRecord {
    type Key = u128;

    fn partition(&self) -> Self::Key {
        self.hash_key
    }
}

#[derive(Debug, Clone, Snafu)]
pub enum Error {
    WorkerDead,
    RecordTooLarge,
    AckDropped,
}
pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug)]
pub(crate) struct KinesisProducer {
    arn: String,
    sender: mpsc::Sender<LineRecord>,
}

impl KinesisProducer {
    pub(crate) fn new(
        client: rusoto_kinesis::KinesisClient,
        arn: String,
        cancel: CancellationToken,
    ) -> (Self, BoxFuture<'static, ()>) {
        let (sender, receiver) = mpsc::channel(100);

        // TODO: Expose these configuration options
        let rps_per_shard = 1500;
        let bps_per_shard = 7 * 1024 * 1024;

        let max_entry_bytes = 1024 * 1024;
        let max_entry_wait = Duration::from_millis(500);
        let max_batch_bytes = 4 * 1014 * 1024;
        let max_batch_records = 500;
        let max_batch_wait = Duration::from_millis(500);
        let retry_backoff = Duration::from_secs(1);

        let (topology, topology_worker) =
            TopologyService::new(client.clone(), arn.clone(), cancel.child_token());

        let (retry, retry_worker) = ErrorHandler::new(
            sender.clone(),
            topology.clone(),
            retry_backoff,
            cancel.child_token(),
        );

        let pipeline_cancel = cancel.child_token();
        let pipeline = ReceiverStream::new(receiver)
            .take_until(async move { pipeline_cancel.cancelled().await })
            .then(move |mut record: LineRecord| {
                let mut topology = topology.clone();
                async move {
                    record.predicted_shard_id = Some(topology.lookup_shard(record.hash_key).await);
                    record
                }
            })
            .partitioned(move || EntryReducer::new(max_entry_bytes), max_entry_wait)
            .partition_limit(
                move || RecordLimiter::new(rps_per_shard, bps_per_shard),
                Duration::from_secs(5),
            )
            .batched(
                RecordBatcher::new(max_batch_bytes, max_batch_records),
                max_batch_wait,
            )
            .map(Ok::<_, ()>)
            .forward(KinesisSink::new(client, arn.clone(), retry));

        let worker = Box::pin(async move {
            let (worker, _, _) = tokio::join!(pipeline, retry_worker, topology_worker);
            worker.unwrap();
        });

        (Self { arn, sender }, worker)
    }

    pub async fn handle_write(&self, lines: Vec<OwnedParsedLine>) -> Result<()> {
        let stream = FuturesUnordered::new();
        for line in lines {
            let (record, orx) = LineRecord::new(line);

            let send_result = self.sender.send(record).await;
            stream.push(async move {
                match send_result {
                    Ok(()) => orx.await.map_err(|_| Error::AckDropped)?,
                    Err(_) => Err(Error::WorkerDead),
                }
            });
        }

        stream.try_collect::<Vec<_>>().await?;
        Ok(())
    }
}
