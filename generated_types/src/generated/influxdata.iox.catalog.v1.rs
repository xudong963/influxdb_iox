/// Represents a parsed predicate for evaluation by the InfluxDB IOx query engine.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Predicate {
    /// Optional table restriction. If present, restricts the results to only tables these tables.
    #[prost(message, optional, tag = "1")]
    pub table_names: ::core::option::Option<OptionalStringSet>,
    /// Optional field restriction. If present, restricts the results to only tables which have *at least one* of the
    /// fields in field_columns.
    #[prost(message, optional, tag = "2")]
    pub field_columns: ::core::option::Option<OptionalStringSet>,
    /// Optional partition key filter
    #[prost(message, optional, tag = "3")]
    pub partition_key: ::core::option::Option<OptionalString>,
    /// Optional timestamp range: only rows within this range are included in results. Other rows are excluded.
    #[prost(message, optional, tag = "4")]
    pub range: ::core::option::Option<TimestampRange>,
    /// Optional arbitrary predicates, represented as list of expressions applied a logical conjunction (aka they are
    /// 'AND'ed together). Only rows that evaluate to TRUE for all these expressions should be returned. Other rows are
    /// excluded from the results.
    #[prost(message, repeated, tag = "5")]
    pub exprs: ::prost::alloc::vec::Vec<Expr>,
}
/// A optional string set.
///
/// This is used instead of a `repeated string` to differenctiate between "empty set" and "none".
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptionalStringSet {
    #[prost(string, repeated, tag = "1")]
    pub values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// An optional string.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptionalString {
    #[prost(string, tag = "1")]
    pub value: ::prost::alloc::string::String,
}
/// Specifies a continuous range of nanosecond timestamps.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimestampRange {
    /// Start defines the inclusive lower bound.
    #[prost(int64, tag = "1")]
    pub start: i64,
    /// End defines the exclusive upper bound.
    #[prost(int64, tag = "2")]
    pub end: i64,
}
/// Single expression to be used as parts of a predicate.
///
/// Only very simple expression of the type `<column> <op> <scalar>` are supported.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Expr {
    /// Column (w/o table name).
    #[prost(string, tag = "1")]
    pub column: ::prost::alloc::string::String,
    /// Operator.
    #[prost(enumeration = "Op", tag = "2")]
    pub op: i32,
    /// Scalar value.
    #[prost(message, optional, tag = "3")]
    pub scalar: ::core::option::Option<Scalar>,
}
/// Scalar value of a certain type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Scalar {
    #[prost(oneof = "scalar::Value", tags = "1, 2, 3, 4")]
    pub value: ::core::option::Option<scalar::Value>,
}
/// Nested message and enum types in `Scalar`.
pub mod scalar {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        #[prost(bool, tag = "1")]
        ValueBool(bool),
        #[prost(int64, tag = "2")]
        ValueI64(i64),
        #[prost(double, tag = "3")]
        ValueF64(f64),
        #[prost(string, tag = "4")]
        ValueString(::prost::alloc::string::String),
    }
}
/// Binary operator that can be evaluated on a column and a scalar value.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Op {
    /// Unspecified operator, will result in an error.
    Unspecified = 0,
    /// Strict equality (`=`).
    Eq = 1,
    /// Inequality (`!=`).
    Ne = 2,
}
/// Path for object store interaction.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Path {
    /// Directory hierarchy.
    #[prost(string, repeated, tag = "1")]
    pub directories: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// File name.
    #[prost(string, tag = "2")]
    pub file_name: ::prost::alloc::string::String,
}
/// Upgrades the catalog to a new version.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Upgrade {
    /// Format string describing the next catalog version.
    #[prost(string, tag = "1")]
    pub format: ::prost::alloc::string::String,
}
/// Adds a [Parquet] file to the catalog.
///
/// [Parquet]: https://parquet.apache.org/
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddParquet {
    /// Path of the file within the object store.
    #[prost(message, optional, tag = "1")]
    pub path: ::core::option::Option<Path>,
    /// The total size of the parquet file, in bytes
    #[prost(uint64, tag = "3")]
    pub file_size_bytes: u64,
    /// [Apache Parquet] metadata encoded using [Apache Thrift].
    ///
    /// The metadata is encoded using the [Thrift Compact Protocol] and compressed using [Zstandard].
    ///
    /// [Apache Parquet]: https://parquet.apache.org/
    /// [Apache Thrift]: https://thrift.apache.org/
    /// [Thrift Compact Protocol]: https://github.com/apache/thrift/blob/master/doc/specs/thrift-compact-protocol.md
    /// [Zstandard]: http://facebook.github.io/zstd/
    #[prost(bytes = "bytes", tag = "2")]
    pub metadata: ::prost::bytes::Bytes,
}
/// Removes a [Parquet] file from the catalog.
///
/// [Parquet]: https://parquet.apache.org/
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveParquet {
    /// Path of the file within the object store.
    #[prost(message, optional, tag = "1")]
    pub path: ::core::option::Option<Path>,
}
/// Chunk within the preserved part of the catalog.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChunkAddr {
    /// Table name.
    #[prost(string, tag = "1")]
    pub table_name: ::prost::alloc::string::String,
    /// Partition key.
    #[prost(string, tag = "2")]
    pub partition_key: ::prost::alloc::string::String,
    /// Chunk ID.
    #[prost(uint32, tag = "3")]
    pub chunk_id: u32,
}
/// Register new delete predicate
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePredicate {
    /// Predicate to be applied.
    #[prost(message, optional, tag = "1")]
    pub predicate: ::core::option::Option<Predicate>,
    /// Chunks that are affected by the predicate.
    #[prost(message, repeated, tag = "2")]
    pub chunks: ::prost::alloc::vec::Vec<ChunkAddr>,
}
/// Single, self-contained transaction.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transaction {
    /// Transaction format version.
    #[prost(uint32, tag = "1")]
    pub version: u32,
    /// Ordered list of actions that are part of this transaction.
    #[prost(message, repeated, tag = "2")]
    pub actions: ::prost::alloc::vec::Vec<transaction::Action>,
    /// Revision counter, must by "previous revision" + 1 or 0 for the first transaction.
    #[prost(uint64, tag = "3")]
    pub revision_counter: u64,
    /// UUID unique to this transaction. Used to detect concurrent transactions. For the first transaction this field is
    /// empty.
    #[prost(string, tag = "4")]
    pub uuid: ::prost::alloc::string::String,
    /// UUID of last commit.
    #[prost(string, tag = "5")]
    pub previous_uuid: ::prost::alloc::string::String,
    /// Start timestamp.
    ///
    /// Timestamp of the start of the transaction.
    #[prost(message, optional, tag = "6")]
    pub start_timestamp: ::core::option::Option<::pbjson_types::Timestamp>,
    /// Definition on how this transaction is encoded.
    #[prost(enumeration = "transaction::Encoding", tag = "7")]
    pub encoding: i32,
}
/// Nested message and enum types in `Transaction`.
pub mod transaction {
    /// Action as part of the transaction, wraps an enum.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Action {
        /// Inner enum.
        #[prost(oneof = "action::Action", tags = "1, 2, 3, 4")]
        pub action: ::core::option::Option<action::Action>,
    }
    /// Nested message and enum types in `Action`.
    pub mod action {
        /// Inner enum.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Action {
            #[prost(message, tag = "1")]
            Upgrade(super::super::Upgrade),
            #[prost(message, tag = "2")]
            AddParquet(super::super::AddParquet),
            #[prost(message, tag = "3")]
            RemoveParquet(super::super::RemoveParquet),
            #[prost(message, tag = "4")]
            DeletePredicate(super::super::DeletePredicate),
        }
    }
    /// Definition of how this transaction relates to previous transaction and how it should be processed.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Encoding {
        /// Unspecified encoding, will lead to an controlled error. Every transaction object MUST decide if the encoding
        /// is "delta" or "full".
        Unspecified = 0,
        /// The actions in this message only encode changes to the previous transactions which must be processed
        /// beforehand. This is the default for "ordinary" transactions.
        Delta = 1,
        /// The actions in this message contain the full state of the catalog at this point in time. This is used for checkpoints.
        Full = 2,
    }
}
/// IOx-specific metadata that will be serialized into the file-level key-value Parquet metadata under a single key.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IoxMetadata {
    /// Metadata format version.
    #[prost(uint32, tag = "1")]
    pub version: u32,
    /// Timestamp when this file was created.
    #[prost(message, optional, tag = "2")]
    pub creation_timestamp: ::core::option::Option<::pbjson_types::Timestamp>,
    /// Table that holds this parquet file.
    #[prost(string, tag = "3")]
    pub table_name: ::prost::alloc::string::String,
    /// Partition key of the partition that holds this parquet file.
    #[prost(string, tag = "4")]
    pub partition_key: ::prost::alloc::string::String,
    /// Chunk ID.
    #[prost(uint32, tag = "5")]
    pub chunk_id: u32,
    /// Partition checkpoint with pre-split data for the in this file.
    #[prost(message, optional, tag = "6")]
    pub partition_checkpoint: ::core::option::Option<PartitionCheckpoint>,
    /// Database checkpoint created at the time of the write.
    #[prost(message, optional, tag = "7")]
    pub database_checkpoint: ::core::option::Option<DatabaseCheckpoint>,
    /// Wallclock timestamp of when the first data in this file was received by IOx.
    #[prost(message, optional, tag = "8")]
    pub time_of_first_write: ::core::option::Option<::pbjson_types::Timestamp>,
    /// Wallclock timestamp of when the last data in this file was received by IOx.
    #[prost(message, optional, tag = "9")]
    pub time_of_last_write: ::core::option::Option<::pbjson_types::Timestamp>,
    /// Order of this chunk relative to other overlapping chunks.
    #[prost(uint32, tag = "10")]
    pub chunk_order: u32,
}
/// Partition checkpoint.
///
/// Note that a partition checkpoint belongs to a single partition (via table name and partition key). Since this
/// checkpoint is usually serialized as part of `IoxMetadata`, the partition information is NOT repeated as part of this
/// message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartitionCheckpoint {
    /// Maps `sequencer_id` to the to-be-persisted minimum and seen maximum sequence numbers.
    #[prost(btree_map = "uint32, message", tag = "1")]
    pub sequencer_numbers: ::prost::alloc::collections::BTreeMap<u32, OptionalMinMaxSequence>,
    /// Minimum unpersisted timestamp.
    #[prost(message, optional, tag = "2")]
    pub min_unpersisted_timestamp: ::core::option::Option<::pbjson_types::Timestamp>,
}
/// Record of the playback state for the whole database.
///
/// This effectively contains the minimum sequence numbers over the whole database that are the starting point for
/// replay.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatabaseCheckpoint {
    /// Maps `sequencer_id` to the to-be-persisted minimum and seen maximum sequence numbers.
    #[prost(btree_map = "uint32, message", tag = "2")]
    pub sequencer_numbers: ::prost::alloc::collections::BTreeMap<u32, OptionalMinMaxSequence>,
}
/// An optional uint64.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptionalUint64 {
    #[prost(uint64, tag = "1")]
    pub value: u64,
}
/// The optional to-be-replayed minimum and seen maximum sequence numbers for a given sequencer.
///
/// If the minimum value is missing, no replay is required for this sequencer.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptionalMinMaxSequence {
    #[prost(message, optional, tag = "1")]
    pub min: ::core::option::Option<OptionalUint64>,
    #[prost(uint64, tag = "2")]
    pub max: u64,
}
