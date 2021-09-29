//! Implementation of a DataFusion PhysicalPlan node across partition chunks

use std::{fmt, sync::Arc, task::{Context, Poll}};

use arrow::{datatypes::SchemaRef, error::Result as ArrowResult, record_batch::RecordBatch};
use datafusion::{error::DataFusionError, physical_plan::{DisplayFormatType, ExecutionPlan, Partitioning, RecordBatchStream, SendableRecordBatchStream, Statistics, metrics::{BaselineMetrics, ExecutionPlanMetricsSet, MetricsSet}}};
use futures::{Stream, StreamExt};
use internal_types::{schema::Schema, selection::Selection};

use crate::QueryChunk;
use predicate::predicate::Predicate;

use async_trait::async_trait;

use super::adapter::SchemaAdapterStream;

/// Implements the DataFusion physical plan interface
#[derive(Debug)]
pub(crate) struct IOxReadFilterNode<C: QueryChunk + 'static> {
    table_name: Arc<str>,
    /// The desired output schema (includes selection)
    /// note that the chunk may not have all these columns.
    iox_schema: Arc<Schema>,
    chunks: Vec<Arc<C>>,
    predicate: Predicate,
    /// Execution metrics
    metrics: ExecutionPlanMetricsSet,
}

impl<C: QueryChunk + 'static> IOxReadFilterNode<C> {
    /// Create a execution plan node that reads data from `chunks` producing
    /// output according to schema, while applying `predicate` and
    /// returns
    pub fn new(
        table_name: Arc<str>,
        iox_schema: Arc<Schema>,
        chunks: Vec<Arc<C>>,
        predicate: Predicate,
    ) -> Self {
        Self {
            table_name,
            iox_schema,
            chunks,
            predicate,
            metrics: ExecutionPlanMetricsSet::new(),
        }
    }
}

#[async_trait]
impl<C: QueryChunk + 'static> ExecutionPlan for IOxReadFilterNode<C> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn schema(&self) -> SchemaRef {
        self.iox_schema.as_arrow()
    }

    fn output_partitioning(&self) -> Partitioning {
        Partitioning::UnknownPartitioning(self.chunks.len())
    }

    fn children(&self) -> Vec<Arc<dyn ExecutionPlan>> {
        // no inputs
        vec![]
    }

    fn with_new_children(
        &self,
        children: Vec<Arc<dyn ExecutionPlan>>,
    ) -> datafusion::error::Result<Arc<dyn ExecutionPlan>> {
        assert!(children.is_empty(), "no children expected in iox plan");

        let chunks: Vec<Arc<C>> = self.chunks.to_vec();

        // For some reason when I used an automatically derived `Clone` implementation
        // the compiler didn't recognize the trait implementation
        let new_self = Self {
            table_name: Arc::clone(&self.table_name),
            iox_schema: Arc::clone(&self.iox_schema),
            chunks,
            predicate: self.predicate.clone(),
            metrics: ExecutionPlanMetricsSet::new(),
        };

        Ok(Arc::new(new_self))
    }

    async fn execute(
        &self,
        partition: usize,
    ) -> datafusion::error::Result<SendableRecordBatchStream> {
        let baseline_metrics = BaselineMetrics::new(&self.metrics, partition);
        let timer = baseline_metrics.elapsed_compute().timer();

        let schema = self.schema();
        let fields = schema.fields();
        let selection_cols = fields.iter().map(|f| f.name() as &str).collect::<Vec<_>>();

        let chunk = Arc::clone(&self.chunks[partition]);

        let chunk_table_schema = chunk.schema();

        // The output selection is all the columns in the schema.
        //
        // However, this chunk may not have all those columns. Thus we
        // restrict the requested selection to the actual columns
        // available, and use SchemaAdapterStream to pad the rest of
        // the columns with NULLs if necessary
        let selection_cols = restrict_selection(selection_cols, &chunk_table_schema);
        let selection = Selection::Some(&selection_cols);

        let del_preds = chunk.delete_predicates();

        let stream = chunk
            .read_filter(&self.predicate, selection, del_preds)
            .map_err(|e| {
                DataFusionError::Execution(format!(
                    "Error creating scan for table {} chunk {}: {}",
                    self.table_name,
                    chunk.id(),
                    e
                ))
            })?;

        // all CPU time is now done, pass in baseline metrics to adapter
        timer.done();

        let adapter = SchemaAdapterStream::try_new(stream, Arc::clone(&schema), baseline_metrics)
            .map_err(|e| DataFusionError::Internal(e.to_string()))?;

        let mut adapter = Box::pin(adapter);

        // HACK: break all record batches into single row (to produce multiple record batches)
        let mut batches = vec![];
        while let Some(batch) = adapter.next().await {
            match batch {
                Err(_) => batches.push(batch),
                Ok(batch) => {
                    println!("Got next batch of {} records", batch.num_rows());
                    for i in 0..batch.num_rows() {
                        // slice into 1 row record batches
                        batches.push(Ok(batch.slice(i, 1)));
                    }
                }
            }
        }
        println!("Created output of {} results", batches.len());

        Ok(Box::pin(MemoryStream { batches , schema }))
    }

    fn fmt_as(&self, t: DisplayFormatType, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match t {
            DisplayFormatType::Default => {
                write!(
                    f,
                    "IOxReadFilterNode: table_name={}, chunks={} predicate={}",
                    self.table_name,
                    self.chunks.len(),
                    self.predicate,
                )
            }
        }
    }

    fn metrics(&self) -> Option<MetricsSet> {
        Some(self.metrics.clone_inner())
    }

    fn statistics(&self) -> Statistics {
        self.chunks
            .iter()
            .fold(None, |combined_summary, chunk| match combined_summary {
                None => Some(chunk.summary().clone()),
                Some(mut combined_summary) => {
                    combined_summary.update_from(chunk.summary());
                    Some(combined_summary)
                }
            })
            .map(|combined_summary| {
                crate::statistics::df_from_iox(self.iox_schema.as_ref(), &combined_summary)
            })
            .unwrap_or_default()
    }
}

/// Removes any columns that are not present in schema, returning a possibly
/// restricted set of columns
fn restrict_selection<'a>(
    selection_cols: Vec<&'a str>,
    chunk_table_schema: &'a Schema,
) -> Vec<&'a str> {
    let arrow_schema = chunk_table_schema.as_arrow();

    selection_cols
        .into_iter()
        .filter(|col| arrow_schema.fields().iter().any(|f| f.name() == col))
        .collect()
}





/// Iterator over batches
pub(crate) struct MemoryStream {
    /// Vector of record batches
    batches: Vec<ArrowResult<RecordBatch>>,
    /// Schema representing the data
    schema: SchemaRef,
}

impl Stream for MemoryStream {
    type Item = ArrowResult<RecordBatch>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        _: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        let next_batch = if !self.batches.is_empty() {
            Some(self.batches.remove(0))
        } else {
            None
        };
        Poll::Ready(next_batch)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.batches.len(), Some(self.batches.len()))
    }
}

impl RecordBatchStream for MemoryStream {
    /// Get the schema
    fn schema(&self) -> SchemaRef {
        self.schema.clone()
    }
}
