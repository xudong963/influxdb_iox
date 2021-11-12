use super::{
    catalog::chunk::ChunkMetadata, pred::to_read_buffer_predicate, streams::ReadFilterResultsStream,
};
use data_types::chunk_metadata::ChunkAddr;
use data_types::{
    chunk_metadata::{ChunkId, ChunkOrder},
    delete_predicate::DeletePredicate,
    partition_metadata,
};
use datafusion::physical_plan::SendableRecordBatchStream;
use datafusion_util::MemoryStream;
use internal_types::access::AccessRecorder;
use iox_object_store::ParquetFilePath;
use mutable_buffer::snapshot::ChunkSnapshot;
use observability_deps::tracing::debug;
use parquet_file::chunk::ParquetChunk;
use partition_metadata::TableSummary;
use predicate::predicate::{Predicate, PredicateMatch};
use query::{exec::stringset::StringSet, QueryChunk, QueryChunkMeta};
use read_buffer::RBChunk;
use schema::InfluxColumnType;
use schema::{selection::Selection, sort::SortKey, Schema};
use snafu::{OptionExt, ResultExt, Snafu};
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
};
use time::Time;

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Mutable Buffer Chunk Error: {}", source))]
    MutableBufferChunk {
        source: mutable_buffer::snapshot::Error,
    },

    #[snafu(display("Read Buffer Error in chunk {}: {}", chunk_id, source))]
    ReadBufferChunkError {
        source: read_buffer::Error,
        chunk_id: ChunkId,
    },

    #[snafu(display("Read Buffer Error in chunk {}: {}", chunk_id, msg))]
    ReadBufferError { chunk_id: ChunkId, msg: String },

    #[snafu(display("Parquet File Error in chunk {}: {}", chunk_id, source))]
    ParquetFileChunkError {
        source: parquet_file::chunk::Error,
        chunk_id: ChunkId,
    },

    #[snafu(display("Internal error restricting schema: {}", source))]
    InternalSelectingSchema { source: schema::Error },

    #[snafu(display("Predicate conversion error: {}", source))]
    PredicateConversion { source: super::pred::Error },

    #[snafu(display(
        "Internal error: mutable buffer does not support predicate pushdown, but got: {:?}",
        predicate
    ))]
    InternalPredicateNotSupported { predicate: Predicate },

    #[snafu(display("internal error creating plan: {}", source))]
    InternalPlanCreation {
        source: datafusion::error::DataFusionError,
    },

    #[snafu(display("arrow conversion error: {}", source))]
    ArrowConversion { source: arrow::error::ArrowError },
}
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// A IOx DatabaseChunk can come from one of three places:
/// MutableBuffer, ReadBuffer, or a ParquetFile
#[derive(Debug)]
pub struct DbChunk {
    addr: ChunkAddr,
    access_recorder: AccessRecorder,
    state: State,
    meta: Arc<ChunkMetadata>,
    time_of_first_write: Time,
    time_of_last_write: Time,
    order: ChunkOrder,
}

#[derive(Debug)]
enum State {
    MutableBuffer { chunk: Arc<ChunkSnapshot> },
    ReadBuffer { chunk: Arc<RBChunk> },
    ParquetFile { chunk: Arc<ParquetChunk> },
}

impl DbChunk {
    /// Create a DBChunk snapshot of the catalog chunk
    pub fn snapshot(chunk: &super::catalog::chunk::CatalogChunk) -> Arc<Self> {
        let addr = chunk.addr().clone();

        use super::catalog::chunk::{ChunkStage, ChunkStageFrozenRepr};

        let (state, meta) = match chunk.stage() {
            ChunkStage::Open { mb_chunk, .. } => {
                let (snapshot, just_cached) = mb_chunk.snapshot();

                // the snapshot might be cached, so we need to update the chunk metrics
                if just_cached {
                    chunk.update_metrics();
                }

                let state = State::MutableBuffer {
                    chunk: Arc::clone(&snapshot),
                };
                let meta = ChunkMetadata {
                    table_summary: Arc::new(mb_chunk.table_summary()),
                    schema: snapshot.full_schema(),
                    delete_predicates: vec![], // open chunk does not have delete predicate
                };
                (state, Arc::new(meta))
            }
            ChunkStage::Frozen {
                representation,
                meta,
                ..
            } => {
                let state = match &representation {
                    ChunkStageFrozenRepr::MutableBufferSnapshot(snapshot) => State::MutableBuffer {
                        chunk: Arc::clone(snapshot),
                    },
                    ChunkStageFrozenRepr::ReadBuffer(repr) => State::ReadBuffer {
                        chunk: Arc::clone(repr),
                    },
                };
                (state, Arc::clone(meta))
            }
            ChunkStage::Persisted {
                parquet,
                read_buffer,
                meta,
                ..
            } => {
                let state = if let Some(read_buffer) = &read_buffer {
                    State::ReadBuffer {
                        chunk: Arc::clone(read_buffer),
                    }
                } else {
                    State::ParquetFile {
                        chunk: Arc::clone(parquet),
                    }
                };
                (state, Arc::clone(meta))
            }
        };

        Arc::new(Self {
            addr,
            access_recorder: chunk.access_recorder().clone(),
            state,
            meta,
            time_of_first_write: chunk.time_of_first_write(),
            time_of_last_write: chunk.time_of_last_write(),
            order: chunk.order(),
        })
    }

    /// Return the snapshot of the chunk with type ParquetFile
    /// This function should be only invoked when you know your chunk
    /// is ParquetFile type whose state is Persisted. The
    /// reason we have this function is because the above snapshot
    /// function always returns the read buffer one for the same state
    pub fn parquet_file_snapshot(chunk: &super::catalog::chunk::CatalogChunk) -> Arc<Self> {
        use super::catalog::chunk::ChunkStage;

        let (state, meta) = match chunk.stage() {
            ChunkStage::Persisted { parquet, meta, .. } => {
                let chunk = Arc::clone(parquet);
                let state = State::ParquetFile { chunk };
                (state, Arc::clone(meta))
            }
            _ => {
                panic!("Internal error: This chunk's stage is not Persisted");
            }
        };
        Arc::new(Self {
            addr: chunk.addr().clone(),
            meta,
            state,
            access_recorder: chunk.access_recorder().clone(),
            time_of_first_write: chunk.time_of_first_write(),
            time_of_last_write: chunk.time_of_last_write(),
            order: chunk.order(),
        })
    }

    /// Return the Path in ObjectStorage where this chunk is
    /// persisted, if any
    pub fn object_store_path(&self) -> Option<&ParquetFilePath> {
        match &self.state {
            State::ParquetFile { chunk } => Some(chunk.path()),
            _ => None,
        }
    }

    /// Return the address of this chunk
    pub fn addr(&self) -> &ChunkAddr {
        &self.addr
    }

    /// Return the name of the table in this chunk
    pub fn table_name(&self) -> &Arc<str> {
        &self.addr.table_name
    }

    pub fn time_of_first_write(&self) -> Time {
        self.time_of_first_write
    }

    pub fn time_of_last_write(&self) -> Time {
        self.time_of_last_write
    }

    /// NOTE: valid Read Buffer predicates are not guaranteed to be applicable
    /// to an arbitrary Read Buffer chunk, because the applicability of a
    /// predicate depends on the schema of the chunk. Callers should validate
    /// predicates against chunks they are to be executed against using
    /// `read_buffer::Chunk::validate_predicate`
    pub fn to_rub_negated_predicates(
        delete_predicates: &[Arc<Predicate>],
    ) -> Result<Vec<read_buffer::Predicate>> {
        let mut rub_preds: Vec<read_buffer::Predicate> = vec![];
        for pred in delete_predicates {
            let rub_pred = to_read_buffer_predicate(pred).context(PredicateConversion)?;
            rub_preds.push(rub_pred);
        }

        debug!(?rub_preds, "RUB delete predicates");
        Ok(rub_preds)
    }

    /// Return true if any of the fields called for in the `predicate`
    /// contain at least 1 null value. Returns false ONLY if all
    /// fields that pass `predicate` are entirely non null
    fn fields_have_nulls(&self, predicate: &Predicate) -> bool {
        self.meta.schema.iter().any(|(influx_column_type, field)| {
            if matches!(influx_column_type, Some(InfluxColumnType::Field(_)))
                && predicate.should_include_field(field.name())
            {
                match self.meta.table_summary.column(field.name()) {
                    Some(column_summary) => {
                        // only if this is false can we return false
                        column_summary.null_count() > 0
                    }
                    None => {
                        // don't know the stats for this column, so assume there can be nulls
                        true
                    }
                }
            } else {
                // not a field column
                false
            }
        })
    }
}

impl QueryChunk for DbChunk {
    type Error = Error;

    fn id(&self) -> ChunkId {
        self.addr.chunk_id
    }

    fn table_name(&self) -> &str {
        self.addr.table_name.as_ref()
    }

    fn may_contain_pk_duplicates(&self) -> bool {
        // Assume that only data in the MUB can contain duplicates
        // within itself as it has the raw incoming stream of writes.
        //
        // All other types of chunks are deduplicated as part of
        // of the reorganization plan run as part of their creation
        matches!(self.state, State::MutableBuffer { .. })
    }

    fn apply_predicate_to_metadata(&self, predicate: &Predicate) -> Result<PredicateMatch> {
        if !predicate.should_include_table(self.table_name().as_ref()) {
            return Ok(PredicateMatch::Zero);
        }

        let pred_result = match &self.state {
            State::MutableBuffer { chunk, .. } => {
                if predicate.has_exprs() || chunk.has_timerange(&predicate.range) {
                    // TODO some more work to figure out if we
                    // definite have / do not have result
                    PredicateMatch::Unknown
                } else {
                    PredicateMatch::Zero
                }
            }
            State::ReadBuffer { chunk, .. } => {
                // If the predicate is not supported by the Read Buffer then
                // it can't determine if the predicate can be answered by
                // meta-data only. A future improvement could be to apply this
                // logic to chunk meta-data without involving the backing
                // execution engine.
                let rb_predicate = match to_read_buffer_predicate(predicate) {
                    Ok(rb_predicate) => rb_predicate,
                    Err(e) => {
                        debug!(?predicate, %e, "read buffer predicate not supported for table_names, falling back");
                        return Ok(PredicateMatch::Unknown);
                    }
                };

                // TODO: currently this will provide an exact answer, which may
                // be expensive in pathological cases. It might make more
                // sense to implement logic that works to rule out chunks based
                // on meta-data only. This should be possible without needing to
                // know the execution engine the chunk is held in.
                if chunk.satisfies_predicate(&rb_predicate) {
                    // if any of the fields referred to in the
                    // predicate has nulls, don't know without more
                    // work if the rows that matched had non null values
                    if self.fields_have_nulls(predicate) {
                        PredicateMatch::Unknown
                    } else {
                        PredicateMatch::AtLeastOneNonNullField
                    }
                } else {
                    PredicateMatch::Zero
                }
            }
            State::ParquetFile { chunk, .. } => {
                if predicate.has_exprs() || chunk.has_timerange(predicate.range.as_ref()) {
                    PredicateMatch::Unknown
                } else {
                    PredicateMatch::Zero
                }
            }
        };

        Ok(pred_result)
    }

    fn read_filter(
        &self,
        predicate: &Predicate,
        selection: Selection<'_>,
        delete_predicates: &[Arc<Predicate>],
    ) -> Result<SendableRecordBatchStream, Self::Error> {
        // Predicate is not required to be applied for correctness. We only pushed it down
        // when possible for performance gain

        debug!(?predicate, "Input Predicate to read_filter");
        self.access_recorder.record_access();

        debug!(?delete_predicates, "Input Delete Predicates to read_filter");

        // merge the negated delete predicates into the select predicate
        let mut pred_with_deleted_exprs = predicate.clone();
        pred_with_deleted_exprs.merge_delete_predicates(delete_predicates);
        debug!(
            ?pred_with_deleted_exprs,
            "Input Predicate plus deleted ranges and deleted predicates"
        );

        match &self.state {
            State::MutableBuffer { chunk, .. } => {
                let batch = chunk.read_filter(selection).context(MutableBufferChunk)?;

                Ok(Box::pin(MemoryStream::new(vec![batch])))
            }
            State::ReadBuffer { chunk, .. } => {
                // Only apply pushdownable predicates
                let rb_predicate = chunk
                    // A predicate unsupported by the Read Buffer or against
                    // this chunk's schema is replaced with a default empty
                    // predicate.
                    .validate_predicate(to_read_buffer_predicate(predicate).unwrap_or_default())
                    .unwrap_or_default();
                debug!(?rb_predicate, "Predicate pushed down to RUB");

                // combine all delete expressions to RUB's negated ones
                let negated_delete_exprs = Self::to_rub_negated_predicates(delete_predicates)?
                    .into_iter()
                    // Any delete predicates unsupported by the Read Buffer will be elided.
                    .filter_map(|p| chunk.validate_predicate(p).ok())
                    .collect::<Vec<_>>();

                debug!(
                    ?negated_delete_exprs,
                    "Negated Predicate pushed down to RUB"
                );

                let read_results = chunk
                    .read_filter(rb_predicate, selection, negated_delete_exprs)
                    .context(ReadBufferChunkError {
                        chunk_id: self.id(),
                    })?;
                let schema =
                    chunk
                        .read_filter_table_schema(selection)
                        .context(ReadBufferChunkError {
                            chunk_id: self.id(),
                        })?;

                Ok(Box::pin(ReadFilterResultsStream::new(
                    read_results,
                    schema.into(),
                )))
            }
            State::ParquetFile { chunk, .. } => chunk
                .read_filter(&pred_with_deleted_exprs, selection)
                .context(ParquetFileChunkError {
                    chunk_id: self.id(),
                }),
        }
    }

    fn column_names(
        &self,
        predicate: &Predicate,
        columns: Selection<'_>,
    ) -> Result<Option<StringSet>, Self::Error> {
        match &self.state {
            State::MutableBuffer { chunk, .. } => {
                if !predicate.is_empty() {
                    // TODO: Support predicates
                    return Ok(None);
                }
                self.access_recorder.record_access();
                Ok(chunk.column_names(columns))
            }
            State::ReadBuffer { chunk, .. } => {
                let rb_predicate = match to_read_buffer_predicate(predicate) {
                    Ok(rb_predicate) => rb_predicate,
                    Err(e) => {
                        debug!(?predicate, %e, "read buffer predicate not supported for column_names, falling back");
                        return Ok(None);
                    }
                };

                self.access_recorder.record_access();
                // TODO(edd): wire up delete predicates to be pushed down to
                // the read buffer.
                Ok(Some(
                    chunk
                        .column_names(rb_predicate, vec![], columns, BTreeSet::new())
                        .context(ReadBufferChunkError {
                            chunk_id: self.id(),
                        })?,
                ))
            }
            State::ParquetFile { chunk, .. } => {
                if !predicate.is_empty() {
                    // TODO: Support predicates when MB supports it
                    return Ok(None);
                }
                self.access_recorder.record_access();
                Ok(chunk.column_names(columns))
            }
        }
    }

    fn column_values(
        &self,
        column_name: &str,
        predicate: &Predicate,
    ) -> Result<Option<StringSet>, Self::Error> {
        match &self.state {
            State::MutableBuffer { .. } => {
                // There is no advantage to manually implementing this
                // vs just letting DataFusion do its thing
                Ok(None)
            }
            State::ReadBuffer { chunk, .. } => {
                let rb_predicate = match to_read_buffer_predicate(predicate) {
                    Ok(rb_predicate) => rb_predicate,
                    Err(e) => {
                        debug!(?predicate, %e, "read buffer predicate not supported for column_names, falling back");
                        return Ok(None);
                    }
                };

                self.access_recorder.record_access();
                let mut values = chunk
                    .column_values(
                        rb_predicate,
                        Selection::Some(&[column_name]),
                        BTreeMap::new(),
                    )
                    .context(ReadBufferChunkError {
                        chunk_id: self.id(),
                    })?;

                // The InfluxRPC frontend only supports getting column values
                // for one column at a time (this is a restriction on the Influx
                // Read gRPC API too). However, the Read Buffer support multiple
                // columns and will return a map - we just need to pull the
                // column out to get the set of values.
                let values = values
                    .remove(column_name)
                    .with_context(|| ReadBufferError {
                        chunk_id: self.id(),
                        msg: format!(
                            "failed to find column_name {:?} in results of tag_values",
                            column_name
                        ),
                    })?;

                Ok(Some(values))
            }
            State::ParquetFile { .. } => {
                // Since DataFusion can read Parquet, there is no advantage to
                // manually implementing this vs just letting DataFusion do its thing
                Ok(None)
            }
        }
    }

    /// Returns true if the chunk is sorted on its pk
    /// Since data is compacted prior being moved to RUBs, data in RUBs and OBs
    /// should be sorted on their PK as the results of compacting.
    /// However, since we current sorted data based on their cardinality (see compute_sort_key),
    /// 2 different chunks may be sorted on different order of key columns.
    fn is_sorted_on_pk(&self) -> bool {
        self.schema().is_sorted_on_pk()
    }

    /// Returns the sort key of the chunk if any
    fn sort_key(&self) -> Option<SortKey<'_>> {
        self.meta.schema.sort_key()
    }

    fn chunk_type(&self) -> &str {
        match &self.state {
            State::MutableBuffer { .. } => "MUB",
            State::ReadBuffer { .. } => "RUB",
            State::ParquetFile { .. } => "OS",
        }
    }

    fn order(&self) -> ChunkOrder {
        self.order
    }
}

impl QueryChunkMeta for DbChunk {
    fn summary(&self) -> &TableSummary {
        self.meta.table_summary.as_ref()
    }

    fn schema(&self) -> Arc<Schema> {
        Arc::clone(&self.meta.schema)
    }

    // return a reference to delete predicates of the chunk
    fn delete_predicates(&self) -> &[Arc<DeletePredicate>] {
        let pred = &self.meta.delete_predicates;
        debug!(?pred, "Delete predicate in  DbChunk");

        pred
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::{
        catalog::chunk::{CatalogChunk, ChunkStage},
        test_helpers::write_lp,
    };
    use crate::utils::make_db_time;
    use data_types::chunk_metadata::ChunkStorage;
    use std::time::Duration;

    async fn test_chunk_access(chunk: &CatalogChunk, time: Arc<time::MockProvider>) {
        let m1 = chunk.access_recorder().get_metrics();
        let snapshot = DbChunk::snapshot(chunk);
        let m2 = chunk.access_recorder().get_metrics();

        let t1 = time.inc(Duration::from_secs(1));
        snapshot
            .read_filter(&Default::default(), Selection::All, &[])
            .unwrap();
        let m3 = chunk.access_recorder().get_metrics();

        let t2 = time.inc(Duration::from_secs(1));
        let column_names = snapshot
            .column_names(&Default::default(), Selection::All)
            .unwrap()
            .is_some();
        let m4 = chunk.access_recorder().get_metrics();

        let t3 = time.inc(Duration::from_secs(1));
        let column_values = snapshot
            .column_values("tag", &Default::default())
            .unwrap()
            .is_some();
        let m5 = chunk.access_recorder().get_metrics();

        // Snapshot shouldn't count as an access
        assert_eq!(m1, m2);

        // Query should count as an access
        assert_eq!(m2.count + 1, m3.count);
        assert!(m2.last_access < m3.last_access);
        assert_eq!(m3.last_access, t1);

        // If column names successful should record access
        match column_names {
            true => {
                assert_eq!(m3.count + 1, m4.count);
                assert_eq!(m4.last_access, t2);
            }
            false => {
                assert_eq!(m3, m4);
            }
        }

        // If column values successful should record access
        match column_values {
            true => {
                assert_eq!(m4.count + 1, m5.count);
                assert!(m4.last_access < m5.last_access);
                assert_eq!(m5.last_access, t3);
            }
            false => {
                assert_eq!(m4, m5);
            }
        }
    }

    #[tokio::test]
    async fn mub_records_access() {
        let (db, time) = make_db_time().await;

        write_lp(&db, "cpu,tag=1 bar=1 1").await;

        let chunks = db.catalog.chunks();
        assert_eq!(chunks.len(), 1);
        let chunk = chunks.into_iter().next().unwrap();
        let chunk = chunk.read();
        assert_eq!(chunk.storage().1, ChunkStorage::OpenMutableBuffer);

        test_chunk_access(&chunk, time).await;
    }

    #[tokio::test]
    async fn rub_records_access() {
        let (db, time) = make_db_time().await;

        write_lp(&db, "cpu,tag=1 bar=1 1").await;
        db.compact_partition("cpu", "1970-01-01T00").await.unwrap();

        let chunks = db.catalog.chunks();
        assert_eq!(chunks.len(), 1);
        let chunk = chunks.into_iter().next().unwrap();
        let chunk = chunk.read();
        assert_eq!(chunk.storage().1, ChunkStorage::ReadBuffer);

        test_chunk_access(&chunk, time).await
    }

    #[tokio::test]
    async fn parquet_records_access() {
        let (db, time) = make_db_time().await;

        let t0 = time.inc(Duration::from_secs(324));
        write_lp(&db, "cpu,tag=1 bar=1 1").await;

        let id = db
            .persist_partition("cpu", "1970-01-01T00", true)
            .await
            .unwrap()
            .unwrap()
            .id();

        db.unload_read_buffer("cpu", "1970-01-01T00", id).unwrap();

        let chunks = db.catalog.chunks();
        assert_eq!(chunks.len(), 1);
        let chunk = chunks.into_iter().next().unwrap();
        let chunk = chunk.read();
        assert_eq!(chunk.storage().1, ChunkStorage::ObjectStoreOnly);

        let first_write = chunk.time_of_first_write();
        let last_write = chunk.time_of_last_write();
        assert_eq!(first_write, t0);
        assert_eq!(last_write, t0);

        test_chunk_access(&chunk, time).await
    }

    #[tokio::test]
    async fn parquet_snapshot() {
        let (db, time) = make_db_time().await;

        let w0 = time.inc(Duration::from_secs(10));
        write_lp(&db, "cpu,tag=1 bar=1 1").await;

        let w1 = time.inc(Duration::from_secs(10));
        write_lp(&db, "cpu,tag=2 bar=2 2").await;

        db.persist_partition("cpu", "1970-01-01T00", true)
            .await
            .unwrap();

        let chunks = db.catalog.chunks();
        assert_eq!(chunks.len(), 1);
        let chunk = chunks.into_iter().next().unwrap();
        let chunk = chunk.read();
        assert!(matches!(chunk.stage(), ChunkStage::Persisted { .. }));
        let snapshot = DbChunk::parquet_file_snapshot(&chunk);

        let first_write = snapshot.time_of_first_write();
        let last_write = snapshot.time_of_last_write();
        assert_eq!(w0, first_write);
        assert_eq!(w1, last_write);
    }
}
