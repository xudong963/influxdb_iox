//! DML data types

#![deny(rustdoc::broken_intra_doc_links, rustdoc::bare_urls, rust_2018_idioms)]
#![warn(
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    clippy::explicit_iter_loop,
    clippy::future_not_send,
    clippy::use_self,
    clippy::clone_on_ref_ptr
)]

use hashbrown::HashMap;
use std::collections::BTreeSet;

use data_types::database_rules::{ShardConfig, ShardId, Sharder};
use data_types::partition_metadata::{StatValues, Statistics};
use data_types::sequence::Sequence;
use data_types::timestamp::TimestampRange;
use mutable_batch::MutableBatch;
use time::Time;
use trace::ctx::SpanContext;

/// Metadata information about a DML operation
#[derive(Debug, Default, Clone)]
pub struct DmlMeta {
    /// The sequence number associated with this write
    sequence: Option<Sequence>,

    /// When this write was ingested into the write buffer
    producer_ts: Option<Time>,

    /// Optional span context associated w/ this write
    span_ctx: Option<SpanContext>,

    /// Bytes read from the wire
    bytes_read: Option<usize>,
}

impl DmlMeta {
    /// Create a new [`DmlMeta`] for a sequenced operation
    pub fn sequenced(
        sequence: Sequence,
        producer_ts: Time,
        span_ctx: Option<SpanContext>,
        bytes_read: usize,
    ) -> Self {
        Self {
            sequence: Some(sequence),
            producer_ts: Some(producer_ts),
            span_ctx,
            bytes_read: Some(bytes_read),
        }
    }

    /// Create a new [`DmlMeta`] for an unsequenced operation
    pub fn unsequenced(span_ctx: Option<SpanContext>) -> Self {
        Self {
            sequence: None,
            producer_ts: None,
            span_ctx,
            bytes_read: None,
        }
    }

    /// Gets the sequence number associated with the write if any
    pub fn sequence(&self) -> Option<&Sequence> {
        self.sequence.as_ref()
    }

    /// Gets the producer timestamp associated with the write if any
    pub fn producer_ts(&self) -> Option<Time> {
        self.producer_ts
    }

    /// Gets the span context if any
    pub fn span_context(&self) -> Option<&SpanContext> {
        self.span_ctx.as_ref()
    }

    /// Returns the number of bytes read from the wire if relevant
    pub fn bytes_read(&self) -> Option<usize> {
        self.bytes_read
    }
}

/// A DML operation
#[derive(Debug, Clone)]
pub enum DmlOperation {
    /// A write operation
    Write(DmlWrite),
}

/// A collection of writes to potentially multiple tables within the same database
#[derive(Debug, Clone)]
pub struct DmlWrite {
    /// Writes to individual tables keyed by table name
    tables: HashMap<String, MutableBatch>,
    /// Write metadata
    meta: DmlMeta,
    min_timestamp: i64,
    max_timestamp: i64,
}

impl DmlWrite {
    /// Create a new [`DmlWrite`]
    ///
    /// # Panic
    ///
    /// Panics if
    ///
    /// - `tables` is empty
    /// - a MutableBatch is empty
    /// - a MutableBatch lacks an i64 "time" column
    pub fn new(tables: HashMap<String, MutableBatch>, meta: DmlMeta) -> Self {
        assert_ne!(tables.len(), 0);

        let mut stats = StatValues::new_empty();
        for (table_name, table) in &tables {
            match table
                .column(schema::TIME_COLUMN_NAME)
                .expect("time")
                .stats()
            {
                Statistics::I64(col_stats) => stats.update_from(&col_stats),
                s => unreachable!(
                    "table \"{}\" has unexpected type for time column: {}",
                    table_name,
                    s.type_name()
                ),
            };
        }

        Self {
            tables,
            meta,
            min_timestamp: stats.min.unwrap(),
            max_timestamp: stats.max.unwrap(),
        }
    }

    /// Metadata associated with this write
    pub fn meta(&self) -> &DmlMeta {
        &self.meta
    }

    /// Set the metadata
    pub fn set_meta(&mut self, meta: DmlMeta) {
        self.meta = meta
    }

    /// Returns an iterator over the per-table writes within this [`DmlWrite`]
    /// in no particular order
    pub fn tables(&self) -> impl Iterator<Item = (&str, &MutableBatch)> + '_ {
        self.tables.iter().map(|(k, v)| (k.as_str(), v))
    }

    /// Gets the write for a given table
    pub fn table(&self, name: &str) -> Option<&MutableBatch> {
        self.tables.get(name)
    }

    /// Returns the number of tables within this write
    pub fn table_count(&self) -> usize {
        self.tables.len()
    }

    /// Returns the minimum timestamp in the write
    pub fn min_timestamp(&self) -> i64 {
        self.min_timestamp
    }

    /// Returns the maximum timestamp in the write
    pub fn max_timestamp(&self) -> i64 {
        self.max_timestamp
    }

    /// Shards this [`DmlWrite`]
    pub fn shard(
        self,
        config: &ShardConfig,
    ) -> Result<HashMap<ShardId, Self>, data_types::database_rules::Error> {
        let mut sharded_tables = HashMap::new();
        for (table, batch) in self.tables {
            let shard = config.shard(&table)?;
            sharded_tables
                .entry(shard)
                .or_insert_with(HashMap::new)
                .insert(table, batch);
        }

        Ok(sharded_tables
            .into_iter()
            .map(|(shard, tables)| (shard, Self::new(tables, self.meta.clone())))
            .collect())
    }
}

/// Represents a parsed delete predicate for evaluation by the InfluxDB IOx
/// query engine.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeletePredicate {
    /// Only rows within this range are included in
    /// results. Other rows are excluded.
    pub range: TimestampRange,

    /// Optional arbitrary predicates, represented as list of
    /// expressions applied a logical conjunction (aka they
    /// are 'AND'ed together). Only rows that evaluate to TRUE for all
    /// these expressions should be returned. Other rows are excluded
    /// from the results.
    pub exprs: Vec<DeleteExpr>,
}

impl DeletePredicate {
    /// Return all columns participating in the expressions of delete predicate minus the time column if any
    pub fn all_column_names_but_time<'a>(&'a self, cols: &mut BTreeSet<&'a str>) {
        for expr in &self.exprs {
            let col = expr.column();
            if col != schema::TIME_COLUMN_NAME {
                cols.insert(col);
            }
        }
    }
}

/// Single expression to be used as parts of a predicate.
///
/// Only very simple expression of the type `<column> <op> <scalar>` are supported.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeleteExpr {
    /// Column (w/o table name).
    pub column: String,

    /// Operator.
    pub op: Op,

    /// Scalar value.
    pub scalar: Scalar,
}

impl DeleteExpr {
    /// Create a new [`DeleteExpr`]
    pub fn new(column: String, op: Op, scalar: Scalar) -> Self {
        Self { column, op, scalar }
    }

    /// Column (w/o table name).
    pub fn column(&self) -> &str {
        &self.column
    }

    /// Operator.
    pub fn op(&self) -> Op {
        self.op
    }

    /// Scalar value.
    pub fn scalar(&self) -> &Scalar {
        &self.scalar
    }
}

impl std::fmt::Display for DeleteExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", self.column(), self.op(), self.scalar())
    }
}


/// Binary operator that can be evaluated on a column and a scalar value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Op {
    /// Strict equality (`=`).
    Eq,

    /// Inequality (`!=`).
    Ne,
}

impl std::fmt::Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::Eq => write!(f, "="),
            Op::Ne => write!(f, "!="),
        }
    }
}


/// Scalar value of a certain type.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(missing_docs)]
pub enum Scalar {
    Bool(bool),
    I64(i64),
    F64(ordered_float::OrderedFloat<f64>),
    String(String),
}

impl std::fmt::Display for Scalar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Scalar::Bool(value) => value.fmt(f),
            Scalar::I64(value) => value.fmt(f),
            Scalar::F64(value) => value.fmt(f),
            Scalar::String(value) => write!(f, "'{}'", value),
        }
    }
}

/// Test utilities
pub mod test_util {
    use arrow_util::display::pretty_format_batches;
    use schema::selection::Selection;

    use super::*;

    /// Asserts two writes are equal
    pub fn assert_writes_eq(a: &DmlWrite, b: &DmlWrite) {
        assert_eq!(a.meta().sequence(), b.meta().sequence());
        assert_eq!(a.meta().producer_ts(), b.meta().producer_ts());

        assert_eq!(a.table_count(), b.table_count());

        for (table_name, a_batch) in a.tables() {
            let b_batch = b.table(table_name).expect("table not found");

            assert_eq!(
                pretty_format_batches(&[a_batch.to_arrow(Selection::All).unwrap()]).unwrap(),
                pretty_format_batches(&[b_batch.to_arrow(Selection::All).unwrap()]).unwrap(),
                "batches for table \"{}\" differ",
                table_name
            );
        }
    }
}
