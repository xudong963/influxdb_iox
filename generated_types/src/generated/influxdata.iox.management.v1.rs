// NOTE: documentation is manually synced from data_types/src/database_rules.rs

/// `ShardConfig` defines rules for assigning a line/row to an individual
/// host or a group of hosts. A shard
/// is a logical concept, but the usage is meant to split data into
/// mutually exclusive areas. The rough order of organization is:
/// database -> shard -> partition -> chunk. For example, you could shard
/// based on table name and assign to 1 of 10 shards. Within each
/// shard you would have partitions, which would likely be based off time.
/// This makes it possible to horizontally scale out writes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShardConfig {
    //// Each matcher, if any, is evaluated in order.
    //// If there is a match, the route will be evaluated to
    //// the given targets, otherwise the hash ring will be evaluated.
    //// This is useful for overriding the hashring function on some hot spot. For
    //// example, if you use the table name as the input to the hash function
    //// and your ring has 4 slots. If two tables that are very hot get
    //// assigned to the same slot you can override that by putting in a
    //// specific matcher to pull that table over to a different node.
    #[prost(message, repeated, tag = "1")]
    pub specific_targets: ::prost::alloc::vec::Vec<MatcherToShard>,
    //// An optional default hasher which will route to one in a collection of
    //// nodes.
    #[prost(message, optional, tag = "2")]
    pub hash_ring: ::core::option::Option<HashRing>,
    //// If set to true the router will ignore any errors sent by the remote
    //// targets in this route. That is, the write request will succeed
    //// regardless of this route's success.
    #[prost(bool, tag = "3")]
    pub ignore_errors: bool,
    //// Mapping between shard IDs and node groups. Other sharding rules use
    //// ShardId as targets.
    #[prost(map = "uint32, message", tag = "4")]
    pub shards: ::std::collections::HashMap<u32, Sink>,
}
/// Maps a matcher with specific shard. If the line/row matches
/// it should be sent to the group.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MatcherToShard {
    #[prost(message, optional, tag = "1")]
    pub matcher: ::core::option::Option<Matcher>,
    #[prost(uint32, tag = "2")]
    pub shard: u32,
}
//// A matcher is used to match routing rules or subscriptions on a row-by-row
//// (or line) basis.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Matcher {
    /// if provided, match if the table name matches against the regex
    #[prost(string, tag = "1")]
    pub table_name_regex: ::prost::alloc::string::String,
    /// paul: what should we use for predicate matching here against a single row/line?
    #[prost(string, tag = "2")]
    pub predicate: ::prost::alloc::string::String,
}
/// Configuration for a specific sink
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sink {
    #[prost(oneof = "sink::Sink", tags = "1, 2, 3")]
    pub sink: ::core::option::Option<sink::Sink>,
}
/// Nested message and enum types in `Sink`.
pub mod sink {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Sink {
        #[prost(message, tag = "1")]
        Iox(super::NodeGroup),
        #[prost(message, tag = "2")]
        Kafka(super::KafkaProducer),
        #[prost(message, tag = "3")]
        DevNull(super::DevNull),
    }
}
/// A collection of IOx nodes
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeGroup {
    #[prost(message, repeated, tag = "1")]
    pub nodes: ::prost::alloc::vec::Vec<node_group::Node>,
}
/// Nested message and enum types in `NodeGroup`.
pub mod node_group {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Node {
        #[prost(uint32, tag = "1")]
        pub id: u32,
    }
}
/// Kafka producer configuration
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KafkaProducer {}
/// Everything sent to /dev/null can eventually be retrieved from /dev/random, given sufficient time.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DevNull {}
/// HashRing is a rule for creating a hash key for a row and mapping that to
/// an individual node on a ring.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HashRing {
    /// If true the table name will be included in the hash key
    #[prost(bool, tag = "1")]
    pub table_name: bool,
    /// include the values of these columns in the hash key
    #[prost(string, repeated, tag = "2")]
    pub columns: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// ring of shards.
    #[prost(uint32, repeated, tag = "3")]
    pub shards: ::prost::alloc::vec::Vec<u32>,
}
/// `PartitionTemplate` is used to compute the partition key of each row that
/// gets written. It can consist of the table name, a column name and its value,
/// a formatted time, or a string column and regex captures of its value. For
/// columns that do not appear in the input row, a blank value is output.
///
/// The key is constructed in order of the template parts; thus ordering changes
/// what partition key is generated.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartitionTemplate {
    #[prost(message, repeated, tag = "1")]
    pub parts: ::prost::alloc::vec::Vec<partition_template::Part>,
}
/// Nested message and enum types in `PartitionTemplate`.
pub mod partition_template {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Part {
        #[prost(oneof = "part::Part", tags = "1, 2, 3, 4, 5")]
        pub part: ::core::option::Option<part::Part>,
    }
    /// Nested message and enum types in `Part`.
    pub mod part {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ColumnFormat {
            #[prost(string, tag = "1")]
            pub column: ::prost::alloc::string::String,
            #[prost(string, tag = "2")]
            pub format: ::prost::alloc::string::String,
        }
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Part {
            #[prost(message, tag = "1")]
            Table(::pbjson_types::Empty),
            #[prost(string, tag = "2")]
            Column(::prost::alloc::string::String),
            #[prost(string, tag = "3")]
            Time(::prost::alloc::string::String),
            #[prost(message, tag = "4")]
            Regex(ColumnFormat),
            #[prost(message, tag = "5")]
            StrfTime(ColumnFormat),
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LifecycleRules {
    /// Once the total amount of buffered data in memory reaches this size start
    /// dropping data from memory
    ///
    /// If 0, no limit
    #[prost(uint64, tag = "4")]
    pub buffer_size_soft: u64,
    /// Once the amount of data in memory reaches this size start
    /// rejecting writes
    ///
    /// If 0, no limit
    #[prost(uint64, tag = "5")]
    pub buffer_size_hard: u64,
    /// Persists chunks to object storage.
    #[prost(bool, tag = "9")]
    pub persist: bool,
    /// Do not allow writing new data to this database
    #[prost(bool, tag = "8")]
    pub immutable: bool,
    /// If the background worker doesn't find any work to do it will
    /// sleep for this many milliseconds before looking again
    ///
    /// If 0, the default backoff is used
    /// See server::db::lifecycle::DEFAULT_LIFECYCLE_BACKOFF
    #[prost(uint64, tag = "10")]
    pub worker_backoff_millis: u64,
    /// After how many transactions should IOx write a new checkpoint?
    ///
    /// If 0 / absent, this default to 100.
    #[prost(uint64, tag = "11")]
    pub catalog_transactions_until_checkpoint: u64,
    /// Prune catalog transactions older than the given age.
    ///
    /// Keeping old transaction can be useful for debugging.
    ///
    /// Defaults to 1 day.
    #[prost(message, optional, tag = "19")]
    pub catalog_transaction_prune_age: ::core::option::Option<::pbjson_types::Duration>,
    /// Once a partition hasn't received a write for this period of time,
    /// it will be compacted and, if set, persisted. Writers will generally
    /// have this amount of time to send late arriving writes or this could
    /// be their clock skew.
    ///
    /// If 0, a server-side default is used
    #[prost(uint32, tag = "12")]
    pub late_arrive_window_seconds: u32,
    /// Maximum number of rows before triggering persistence
    ///
    /// If 0, a server-side default is used
    #[prost(uint64, tag = "13")]
    pub persist_row_threshold: u64,
    /// Maximum age of a write before triggering persistence
    ///
    /// If 0, a server-side default is used
    #[prost(uint32, tag = "14")]
    pub persist_age_threshold_seconds: u32,
    /// Maximum number of rows to buffer in a MUB chunk before compacting it
    ///
    /// If 0, a server-side default is used
    #[prost(uint64, tag = "15")]
    pub mub_row_threshold: u64,
    /// Use up to this amount of space in bytes for caching Parquet files.
    /// A value of 0 disables Parquet caching
    #[prost(uint64, tag = "17")]
    pub parquet_cache_limit: u64,
    /// If not specified a server-side default is used
    #[prost(oneof = "lifecycle_rules::MaxActiveCompactionsCfg", tags = "16, 18")]
    pub max_active_compactions_cfg:
        ::core::option::Option<lifecycle_rules::MaxActiveCompactionsCfg>,
}
/// Nested message and enum types in `LifecycleRules`.
pub mod lifecycle_rules {
    /// If not specified a server-side default is used
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum MaxActiveCompactionsCfg {
        /// The maximum number of concurrent active compactions that can run.
        #[prost(uint32, tag = "16")]
        MaxActiveCompactions(u32),
        /// The maximum number of concurrent active compactions that can run
        /// expressed as a fraction of the available cpus (rounded to the next smallest non-zero integer).
        #[prost(float, tag = "18")]
        MaxActiveCompactionsCpuFraction(f32),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatabaseRules {
    /// The unencoded name of the database
    ///
    /// Must be a non-empty string containing no control characters
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Template that generates a partition key for each row inserted into the database
    ///
    /// If not specified, a server-side default is used
    #[prost(message, optional, tag = "2")]
    pub partition_template: ::core::option::Option<PartitionTemplate>,
    /// Configures how data flows through the system
    ///
    /// If not specified, a server-side default is used
    #[prost(message, optional, tag = "3")]
    pub lifecycle_rules: ::core::option::Option<LifecycleRules>,
    /// Duration for which the cleanup loop should sleep on average.
    /// Defaults to 500 seconds.
    #[prost(message, optional, tag = "10")]
    pub worker_cleanup_avg_sleep: ::core::option::Option<::pbjson_types::Duration>,
    /// Optionally, the connection for the write buffer for writing or reading/restoring data.
    ///
    /// If not specified, does not configure a write buffer
    #[prost(message, optional, tag = "13")]
    pub write_buffer_connection: ::core::option::Option<WriteBufferConnection>,
    /// If not specified, does not configure any routing
    #[prost(oneof = "database_rules::RoutingRules", tags = "8, 9")]
    pub routing_rules: ::core::option::Option<database_rules::RoutingRules>,
}
/// Nested message and enum types in `DatabaseRules`.
pub mod database_rules {
    /// If not specified, does not configure any routing
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RoutingRules {
        /// Shard config
        #[prost(message, tag = "8")]
        ShardConfig(super::ShardConfig),
        /// Routing config
        #[prost(message, tag = "9")]
        RoutingConfig(super::RoutingConfig),
    }
}
/// Configures the use of a write buffer.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteBufferConnection {
    /// If the buffer is used for reading or writing.
    #[prost(enumeration = "write_buffer_connection::Direction", tag = "1")]
    pub direction: i32,
    /// Which type should be used (e.g. "kafka", "mock")
    #[prost(string, tag = "2")]
    pub r#type: ::prost::alloc::string::String,
    /// Connection string, depends on `type`.
    #[prost(string, tag = "3")]
    pub connection: ::prost::alloc::string::String,
    /// Special configs to be applied when establishing the connection.
    ///
    /// This depends on `type` and can configure aspects like timeouts.
    #[prost(map = "string, string", tag = "6")]
    pub connection_config:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Specifies if the sequencers (e.g. for Kafka in form of a topic w/ `n_sequencers` partitions) should be
    /// automatically created if they do not existing prior to reading or writing.
    #[prost(message, optional, tag = "8")]
    pub creation_config: ::core::option::Option<WriteBufferCreationConfig>,
}
/// Nested message and enum types in `WriteBufferConnection`.
pub mod write_buffer_connection {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Direction {
        /// Unspecified direction, will be treated as an error.
        Unspecified = 0,
        /// Writes into the buffer aka "producer".
        Write = 1,
        /// Reads from the buffer aka "consumer".
        Read = 2,
    }
}
/// Configs sequencer auto-creation for write buffers.
///
/// What that means depends on the used write buffer, e.g. for Kafka this will create a new topic w/ `n_sequencers`
/// partitions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteBufferCreationConfig {
    /// Number of sequencers.
    ///
    /// How they are implemented depends on `type`, e.g. for Kafka this is mapped to the number of partitions.
    ///
    /// If 0, a server-side default is used
    #[prost(uint32, tag = "1")]
    pub n_sequencers: u32,
    /// Special configs to by applied when sequencers are created.
    ///
    /// This depends on `type` and can setup parameters like retention policy.
    ///
    /// Contains 0 or more key value pairs
    #[prost(map = "string, string", tag = "2")]
    pub options:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoutingConfig {
    #[prost(message, optional, tag = "2")]
    pub sink: ::core::option::Option<Sink>,
}
/// `Chunk` represents part of a partition of data in a database.
/// A chunk can contain one or more tables.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Chunk {
    /// The partition key of this chunk
    #[prost(string, tag = "1")]
    pub partition_key: ::prost::alloc::string::String,
    /// The table of this chunk
    #[prost(string, tag = "8")]
    pub table_name: ::prost::alloc::string::String,
    /// The id of this chunk
    #[prost(uint32, tag = "2")]
    pub id: u32,
    /// Which storage system the chunk is located in
    #[prost(enumeration = "ChunkStorage", tag = "3")]
    pub storage: i32,
    /// Is there any outstanding lifecycle action for this chunk?
    #[prost(enumeration = "ChunkLifecycleAction", tag = "10")]
    pub lifecycle_action: i32,
    /// The number of bytes used to store this chunk in memory
    #[prost(uint64, tag = "4")]
    pub memory_bytes: u64,
    /// The number of bytes used to store this chunk in object storage
    #[prost(uint64, tag = "11")]
    pub object_store_bytes: u64,
    /// The number of rows in this chunk
    #[prost(uint64, tag = "9")]
    pub row_count: u64,
    /// The time at which the chunk data was accessed, by a query or a write
    #[prost(message, optional, tag = "12")]
    pub time_of_last_access: ::core::option::Option<::pbjson_types::Timestamp>,
    /// The earliest time at which data contained within this chunk was written
    /// into IOx. Note due to the compaction, etc... this may not be the chunk
    /// that data was originally written into
    #[prost(message, optional, tag = "5")]
    pub time_of_first_write: ::core::option::Option<::pbjson_types::Timestamp>,
    /// The latest time at which data contained within this chunk was written
    /// into IOx. Note due to the compaction, etc... this may not be the chunk
    /// that data was originally written into
    #[prost(message, optional, tag = "6")]
    pub time_of_last_write: ::core::option::Option<::pbjson_types::Timestamp>,
    /// Time at which this chunk was marked as closed. Note this is not
    /// the same as the timestamps on the data itself
    #[prost(message, optional, tag = "7")]
    pub time_closed: ::core::option::Option<::pbjson_types::Timestamp>,
    /// Order of this chunk relative to other overlapping chunks.
    #[prost(uint32, tag = "13")]
    pub order: u32,
}
/// Which storage system is a chunk located in?
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ChunkStorage {
    /// Not currently returned
    Unspecified = 0,
    /// The chunk is still open for new writes, in the Mutable Buffer
    OpenMutableBuffer = 1,
    /// The chunk is no longer open for writes, in the Mutable Buffer
    ClosedMutableBuffer = 2,
    /// The chunk is in the Read Buffer (where it can not be mutated)
    ReadBuffer = 3,
    /// The chunk is in the Read Buffer and Object Store
    ReadBufferAndObjectStore = 4,
    /// The chunk is stored in Object Storage (where it can not be mutated)
    ObjectStoreOnly = 5,
}
/// Is there any lifecycle action currently outstanding for this chunk?
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ChunkLifecycleAction {
    /// No lifecycle
    Unspecified = 0,
    //// Chunk is in the process of being written to object storage
    Persisting = 2,
    //// Chunk is in the process of being compacted
    Compacting = 3,
    //// Chunk is about to be dropped from memory and (if persisted) from object store.
    Dropping = 4,
}
/// `Partition` is comprised of data in one or more chunks
///
/// TODO: add additional information to this structure (e.g. partition
/// names, stats, etc)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Partition {
    /// The partition key of this partition
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetServerIdRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetServerIdResponse {
    /// Must be non-zero
    #[prost(uint32, tag = "1")]
    pub id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateServerIdRequest {
    /// Must be non-zero
    #[prost(uint32, tag = "1")]
    pub id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateServerIdResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetServingReadinessRequest {
    /// If false, the IOx server will respond with UNAVAILABLE to all data plane requests.
    #[prost(bool, tag = "1")]
    pub ready: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetServingReadinessResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDatabasesRequest {
    /// If true, returns only explicitly defined values. See additional
    /// details on `GetDatabaseRequest`.
    #[prost(bool, tag = "1")]
    pub omit_defaults: bool,
}
/// old version of this API returned names only.
/// repeated string names = 1;
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDatabasesResponse {
    //// database rules (configuration) for each database
    #[prost(message, repeated, tag = "2")]
    pub rules: ::prost::alloc::vec::Vec<DatabaseRules>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDatabaseRequest {
    /// The name of the database to retrieve
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// If false: return the current configuration that is being used by
    /// the server, with all server-side default values filled in.
    ///
    /// If true, returns only the persisted configuration (aka only
    /// fields which were was supplied when the database was created or
    /// last modified via UpdateDatabase)
    #[prost(bool, tag = "2")]
    pub omit_defaults: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDatabaseResponse {
    #[prost(message, optional, tag = "1")]
    pub rules: ::core::option::Option<DatabaseRules>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDatabaseRequest {
    #[prost(message, optional, tag = "1")]
    pub rules: ::core::option::Option<DatabaseRules>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDatabaseResponse {}
/// Update a database.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDatabaseRequest {
    /// The rule's `name` field is used to identify the database rules to be updated.
    #[prost(message, optional, tag = "1")]
    pub rules: ::core::option::Option<DatabaseRules>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDatabaseResponse {
    #[prost(message, optional, tag = "1")]
    pub rules: ::core::option::Option<DatabaseRules>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDatabaseRequest {
    /// the name of the database
    #[prost(string, tag = "1")]
    pub db_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDatabaseResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RestoreDatabaseRequest {
    /// The generation ID of the deleted database.
    #[prost(uint64, tag = "1")]
    pub generation_id: u64,
    /// the name of the database
    #[prost(string, tag = "2")]
    pub db_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RestoreDatabaseResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDeletedDatabasesRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDeletedDatabasesResponse {
    #[prost(message, repeated, tag = "1")]
    pub deleted_databases: ::prost::alloc::vec::Vec<DetailedDatabase>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDetailedDatabasesRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDetailedDatabasesResponse {
    #[prost(message, repeated, tag = "1")]
    pub databases: ::prost::alloc::vec::Vec<DetailedDatabase>,
}
/// This resource represents detailed information about a database.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DetailedDatabase {
    /// The generation ID of the database.
    #[prost(uint64, tag = "1")]
    pub generation_id: u64,
    /// The UTC datetime at which this database was deleted, if applicable.
    #[prost(message, optional, tag = "2")]
    pub deleted_at: ::core::option::Option<::pbjson_types::Timestamp>,
    /// The name of the database.
    #[prost(string, tag = "3")]
    pub db_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListChunksRequest {
    /// the name of the database
    #[prost(string, tag = "1")]
    pub db_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListChunksResponse {
    #[prost(message, repeated, tag = "1")]
    pub chunks: ::prost::alloc::vec::Vec<Chunk>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDummyJobRequest {
    #[prost(uint64, repeated, tag = "1")]
    pub nanos: ::prost::alloc::vec::Vec<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDummyJobResponse {
    #[prost(message, optional, tag = "1")]
    pub operation:
        ::core::option::Option<super::super::super::super::google::longrunning::Operation>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRemotesRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRemotesResponse {
    #[prost(message, repeated, tag = "1")]
    pub remotes: ::prost::alloc::vec::Vec<Remote>,
}
/// This resource represents a remote IOx server.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Remote {
    /// The server ID associated with a remote IOx server.
    #[prost(uint32, tag = "1")]
    pub id: u32,
    /// The address of the remote IOx server gRPC endpoint.
    #[prost(string, tag = "2")]
    pub connection_string: ::prost::alloc::string::String,
}
/// Updates information about a remote IOx server.
///
/// If a remote for a given `id` already exists, it is updated in place.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRemoteRequest {
    /// If omitted, the remote associated with `id` will be removed.
    #[prost(message, optional, tag = "1")]
    pub remote: ::core::option::Option<Remote>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRemoteResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRemoteRequest {
    #[prost(uint32, tag = "1")]
    pub id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRemoteResponse {}
/// Request to list all partitions from a named database
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPartitionsRequest {
    /// the name of the database
    #[prost(string, tag = "1")]
    pub db_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPartitionsResponse {
    /// All partitions in a database
    #[prost(message, repeated, tag = "1")]
    pub partitions: ::prost::alloc::vec::Vec<Partition>,
}
/// Request to list all chunks in a specific partitions from a named database
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPartitionChunksRequest {
    /// the name of the database
    #[prost(string, tag = "1")]
    pub db_name: ::prost::alloc::string::String,
    /// the partition key
    #[prost(string, tag = "2")]
    pub partition_key: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPartitionResponse {
    /// Detailed information about a partition
    #[prost(message, optional, tag = "1")]
    pub partition: ::core::option::Option<Partition>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPartitionChunksResponse {
    /// All chunks in a partition
    #[prost(message, repeated, tag = "1")]
    pub chunks: ::prost::alloc::vec::Vec<Chunk>,
}
/// Request to get details of a specific partition from a named database
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPartitionRequest {
    /// the name of the database
    #[prost(string, tag = "1")]
    pub db_name: ::prost::alloc::string::String,
    /// the partition key
    #[prost(string, tag = "2")]
    pub partition_key: ::prost::alloc::string::String,
}
/// Request that a new chunk for writing is created in the mutable buffer
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewPartitionChunkRequest {
    /// the name of the database
    #[prost(string, tag = "1")]
    pub db_name: ::prost::alloc::string::String,
    /// the partition key
    #[prost(string, tag = "2")]
    pub partition_key: ::prost::alloc::string::String,
    /// the table name
    #[prost(string, tag = "3")]
    pub table_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewPartitionChunkResponse {}
/// Request that a chunk be closed and moved to the read buffer
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClosePartitionChunkRequest {
    /// the name of the database
    #[prost(string, tag = "1")]
    pub db_name: ::prost::alloc::string::String,
    /// the partition key
    #[prost(string, tag = "2")]
    pub partition_key: ::prost::alloc::string::String,
    /// the table name
    #[prost(string, tag = "4")]
    pub table_name: ::prost::alloc::string::String,
    /// the chunk id
    #[prost(uint32, tag = "3")]
    pub chunk_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClosePartitionChunkResponse {
    /// The operation that tracks the work for migrating the chunk
    #[prost(message, optional, tag = "1")]
    pub operation:
        ::core::option::Option<super::super::super::super::google::longrunning::Operation>,
}
/// Request to unload chunk from read buffer but keep it in object store
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnloadPartitionChunkRequest {
    /// the name of the database
    #[prost(string, tag = "1")]
    pub db_name: ::prost::alloc::string::String,
    /// the partition key
    #[prost(string, tag = "2")]
    pub partition_key: ::prost::alloc::string::String,
    /// the table name
    #[prost(string, tag = "4")]
    pub table_name: ::prost::alloc::string::String,
    /// the chunk id
    #[prost(uint32, tag = "3")]
    pub chunk_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnloadPartitionChunkResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetServerStatusRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetServerStatusResponse {
    /// Server status.
    #[prost(message, optional, tag = "1")]
    pub server_status: ::core::option::Option<ServerStatus>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerStatus {
    /// Server is initialized, i.e. databases are loaded and accept read/write operations. Furthermore database rules can
    /// be updaded and new databases can be created.
    #[prost(bool, tag = "1")]
    pub initialized: bool,
    /// If present, the server reports a global error condition.
    #[prost(message, optional, tag = "2")]
    pub error: ::core::option::Option<Error>,
    /// If `initialized` is true, this contains a complete list of databases.
    #[prost(message, repeated, tag = "3")]
    pub database_statuses: ::prost::alloc::vec::Vec<DatabaseStatus>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatabaseStatus {
    /// The name of the database.
    #[prost(string, tag = "1")]
    pub db_name: ::prost::alloc::string::String,
    /// If present, the database reports an error condition.
    #[prost(message, optional, tag = "2")]
    pub error: ::core::option::Option<Error>,
    /// Current initialization state of the database.
    #[prost(enumeration = "database_status::DatabaseState", tag = "3")]
    pub state: i32,
}
/// Nested message and enum types in `DatabaseStatus`.
pub mod database_status {
    /// Current initialization state of the database.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum DatabaseState {
        Unspecified = 0,
        /// Database is known but nothing is loaded.
        Known = 1,
        /// Database object storage has been found
        DatabaseObjectStoreFound = 8,
        /// No active database
        NoActiveDatabase = 10,
        /// Rules are loaded
        RulesLoaded = 2,
        /// Catalog is loaded but data from sequencers / write buffers is not yet replayed.
        CatalogLoaded = 3,
        /// Fully initialized database.
        Initialized = 4,
        /// Error loading rules
        RulesLoadError = 5,
        /// Error during catalog load
        CatalogLoadError = 6,
        /// Error during replay
        ReplayError = 7,
        /// Error encountered identifying active generation
        DatabaseObjectStoreLookupError = 9,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Error {
    /// Message describing the error.
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
}
/// Request to wipe preserved catalog.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WipePreservedCatalogRequest {
    /// the name of the database
    #[prost(string, tag = "1")]
    pub db_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WipePreservedCatalogResponse {
    /// The operation that tracks the work for wiping the catalog.
    #[prost(message, optional, tag = "1")]
    pub operation:
        ::core::option::Option<super::super::super::super::google::longrunning::Operation>,
}
/// Request to skip replay.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SkipReplayRequest {
    /// the name of the database
    #[prost(string, tag = "1")]
    pub db_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SkipReplayResponse {}
/// Request to persist given partition.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PersistPartitionRequest {
    /// the name of the database
    #[prost(string, tag = "1")]
    pub db_name: ::prost::alloc::string::String,
    /// the partition key
    #[prost(string, tag = "2")]
    pub partition_key: ::prost::alloc::string::String,
    /// the table name
    #[prost(string, tag = "3")]
    pub table_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PersistPartitionResponse {}
/// Request to drop partition from memory and (if persisted) from object store.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DropPartitionRequest {
    /// the name of the database
    #[prost(string, tag = "1")]
    pub db_name: ::prost::alloc::string::String,
    /// the partition key
    #[prost(string, tag = "2")]
    pub partition_key: ::prost::alloc::string::String,
    /// the table name
    #[prost(string, tag = "3")]
    pub table_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DropPartitionResponse {}
/// Request to delete data from a table on a specified predicate
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRequest {
    /// name of the database
    #[prost(string, tag = "1")]
    pub db_name: ::prost::alloc::string::String,
    /// table name
    #[prost(string, tag = "2")]
    pub table_name: ::prost::alloc::string::String,
    /// start time range
    #[prost(string, tag = "3")]
    pub start_time: ::prost::alloc::string::String,
    /// stop time range
    #[prost(string, tag = "4")]
    pub stop_time: ::prost::alloc::string::String,
    /// predicate
    /// conjunctive expressions of binary 'column_name = literal' or 'column_ame != literal'
    #[prost(string, tag = "5")]
    pub predicate: ::prost::alloc::string::String,
}
/// NGA todo: define an appropriate response
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteResponse {}
#[doc = r" Generated client implementations."]
pub mod management_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct ManagementServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ManagementServiceClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ManagementServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + Sync + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> ManagementServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            ManagementServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        pub async fn get_server_id(
            &mut self,
            request: impl tonic::IntoRequest<super::GetServerIdRequest>,
        ) -> Result<tonic::Response<super::GetServerIdResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/influxdata.iox.management.v1.ManagementService/GetServerId",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn update_server_id(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateServerIdRequest>,
        ) -> Result<tonic::Response<super::UpdateServerIdResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/influxdata.iox.management.v1.ManagementService/UpdateServerId",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn set_serving_readiness(
            &mut self,
            request: impl tonic::IntoRequest<super::SetServingReadinessRequest>,
        ) -> Result<tonic::Response<super::SetServingReadinessResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/influxdata.iox.management.v1.ManagementService/SetServingReadiness",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " List all databases on this server."]
        #[doc = ""]
        #[doc = " Roughly follows the <https://google.aip.dev/132> pattern, except we wrap the response"]
        pub async fn list_databases(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDatabasesRequest>,
        ) -> Result<tonic::Response<super::ListDatabasesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/influxdata.iox.management.v1.ManagementService/ListDatabases",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Return a specific database by name"]
        #[doc = ""]
        #[doc = " Roughly follows the <https://google.aip.dev/131> pattern, except"]
        #[doc = " we wrap the response"]
        pub async fn get_database(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDatabaseRequest>,
        ) -> Result<tonic::Response<super::GetDatabaseResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/influxdata.iox.management.v1.ManagementService/GetDatabase",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn create_database(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDatabaseRequest>,
        ) -> Result<tonic::Response<super::CreateDatabaseResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/influxdata.iox.management.v1.ManagementService/CreateDatabase",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Update a database."]
        #[doc = ""]
        #[doc = " Roughly follows the <https://google.aip.dev/134> pattern, except we wrap the response"]
        pub async fn update_database(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateDatabaseRequest>,
        ) -> Result<tonic::Response<super::UpdateDatabaseResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/influxdata.iox.management.v1.ManagementService/UpdateDatabase",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_database(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteDatabaseRequest>,
        ) -> Result<tonic::Response<super::DeleteDatabaseResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/influxdata.iox.management.v1.ManagementService/DeleteDatabase",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn restore_database(
            &mut self,
            request: impl tonic::IntoRequest<super::RestoreDatabaseRequest>,
        ) -> Result<tonic::Response<super::RestoreDatabaseResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/influxdata.iox.management.v1.ManagementService/RestoreDatabase",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " List deleted databases and their metadata."]
        pub async fn list_deleted_databases(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDeletedDatabasesRequest>,
        ) -> Result<tonic::Response<super::ListDeletedDatabasesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/influxdata.iox.management.v1.ManagementService/ListDeletedDatabases",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " List all databases and their metadata."]
        pub async fn list_detailed_databases(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDetailedDatabasesRequest>,
        ) -> Result<tonic::Response<super::ListDetailedDatabasesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/influxdata.iox.management.v1.ManagementService/ListDetailedDatabases",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " List chunks available on this database"]
        pub async fn list_chunks(
            &mut self,
            request: impl tonic::IntoRequest<super::ListChunksRequest>,
        ) -> Result<tonic::Response<super::ListChunksResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/influxdata.iox.management.v1.ManagementService/ListChunks",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " List remote IOx servers we know about."]
        pub async fn list_remotes(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRemotesRequest>,
        ) -> Result<tonic::Response<super::ListRemotesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/influxdata.iox.management.v1.ManagementService/ListRemotes",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Update information about a remote IOx server (upsert)."]
        pub async fn update_remote(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateRemoteRequest>,
        ) -> Result<tonic::Response<super::UpdateRemoteResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/influxdata.iox.management.v1.ManagementService/UpdateRemote",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Delete a reference to remote IOx server."]
        pub async fn delete_remote(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteRemoteRequest>,
        ) -> Result<tonic::Response<super::DeleteRemoteResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/influxdata.iox.management.v1.ManagementService/DeleteRemote",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a dummy job that for each value of the nanos field"]
        #[doc = " spawns a task that sleeps for that number of nanoseconds before returning"]
        pub async fn create_dummy_job(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDummyJobRequest>,
        ) -> Result<tonic::Response<super::CreateDummyJobResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/influxdata.iox.management.v1.ManagementService/CreateDummyJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " List partitions in a database"]
        pub async fn list_partitions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPartitionsRequest>,
        ) -> Result<tonic::Response<super::ListPartitionsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/influxdata.iox.management.v1.ManagementService/ListPartitions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Get detail information about a partition"]
        pub async fn get_partition(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPartitionRequest>,
        ) -> Result<tonic::Response<super::GetPartitionResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/influxdata.iox.management.v1.ManagementService/GetPartition",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " List chunks in a partition"]
        pub async fn list_partition_chunks(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPartitionChunksRequest>,
        ) -> Result<tonic::Response<super::ListPartitionChunksResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/influxdata.iox.management.v1.ManagementService/ListPartitionChunks",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Create a new chunk in the mutable buffer"]
        pub async fn new_partition_chunk(
            &mut self,
            request: impl tonic::IntoRequest<super::NewPartitionChunkRequest>,
        ) -> Result<tonic::Response<super::NewPartitionChunkResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/influxdata.iox.management.v1.ManagementService/NewPartitionChunk",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Close a chunk and move it to the read buffer"]
        pub async fn close_partition_chunk(
            &mut self,
            request: impl tonic::IntoRequest<super::ClosePartitionChunkRequest>,
        ) -> Result<tonic::Response<super::ClosePartitionChunkResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/influxdata.iox.management.v1.ManagementService/ClosePartitionChunk",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Unload chunk from read buffer but keep it in object store"]
        pub async fn unload_partition_chunk(
            &mut self,
            request: impl tonic::IntoRequest<super::UnloadPartitionChunkRequest>,
        ) -> Result<tonic::Response<super::UnloadPartitionChunkResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/influxdata.iox.management.v1.ManagementService/UnloadPartitionChunk",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Get server status"]
        pub async fn get_server_status(
            &mut self,
            request: impl tonic::IntoRequest<super::GetServerStatusRequest>,
        ) -> Result<tonic::Response<super::GetServerStatusResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/influxdata.iox.management.v1.ManagementService/GetServerStatus",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Wipe preserved catalog for given DB."]
        pub async fn wipe_preserved_catalog(
            &mut self,
            request: impl tonic::IntoRequest<super::WipePreservedCatalogRequest>,
        ) -> Result<tonic::Response<super::WipePreservedCatalogResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/influxdata.iox.management.v1.ManagementService/WipePreservedCatalog",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Skip replay for given DB."]
        pub async fn skip_replay(
            &mut self,
            request: impl tonic::IntoRequest<super::SkipReplayRequest>,
        ) -> Result<tonic::Response<super::SkipReplayResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/influxdata.iox.management.v1.ManagementService/SkipReplay",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Persist given partition."]
        #[doc = ""]
        #[doc = " Errors if there is nothing to persist at the moment as per the lifecycle rules. If successful it returns the"]
        #[doc = " chunk that contains the persisted data."]
        pub async fn persist_partition(
            &mut self,
            request: impl tonic::IntoRequest<super::PersistPartitionRequest>,
        ) -> Result<tonic::Response<super::PersistPartitionResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/influxdata.iox.management.v1.ManagementService/PersistPartition",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Drop partition from memory and (if persisted) from object store."]
        pub async fn drop_partition(
            &mut self,
            request: impl tonic::IntoRequest<super::DropPartitionRequest>,
        ) -> Result<tonic::Response<super::DropPartitionResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/influxdata.iox.management.v1.ManagementService/DropPartition",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Delete data for a table on a specified predicate"]
        pub async fn delete(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteRequest>,
        ) -> Result<tonic::Response<super::DeleteResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/influxdata.iox.management.v1.ManagementService/Delete",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod management_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with ManagementServiceServer."]
    #[async_trait]
    pub trait ManagementService: Send + Sync + 'static {
        async fn get_server_id(
            &self,
            request: tonic::Request<super::GetServerIdRequest>,
        ) -> Result<tonic::Response<super::GetServerIdResponse>, tonic::Status>;
        async fn update_server_id(
            &self,
            request: tonic::Request<super::UpdateServerIdRequest>,
        ) -> Result<tonic::Response<super::UpdateServerIdResponse>, tonic::Status>;
        async fn set_serving_readiness(
            &self,
            request: tonic::Request<super::SetServingReadinessRequest>,
        ) -> Result<tonic::Response<super::SetServingReadinessResponse>, tonic::Status>;
        #[doc = " List all databases on this server."]
        #[doc = ""]
        #[doc = " Roughly follows the <https://google.aip.dev/132> pattern, except we wrap the response"]
        async fn list_databases(
            &self,
            request: tonic::Request<super::ListDatabasesRequest>,
        ) -> Result<tonic::Response<super::ListDatabasesResponse>, tonic::Status>;
        #[doc = " Return a specific database by name"]
        #[doc = ""]
        #[doc = " Roughly follows the <https://google.aip.dev/131> pattern, except"]
        #[doc = " we wrap the response"]
        async fn get_database(
            &self,
            request: tonic::Request<super::GetDatabaseRequest>,
        ) -> Result<tonic::Response<super::GetDatabaseResponse>, tonic::Status>;
        async fn create_database(
            &self,
            request: tonic::Request<super::CreateDatabaseRequest>,
        ) -> Result<tonic::Response<super::CreateDatabaseResponse>, tonic::Status>;
        #[doc = " Update a database."]
        #[doc = ""]
        #[doc = " Roughly follows the <https://google.aip.dev/134> pattern, except we wrap the response"]
        async fn update_database(
            &self,
            request: tonic::Request<super::UpdateDatabaseRequest>,
        ) -> Result<tonic::Response<super::UpdateDatabaseResponse>, tonic::Status>;
        async fn delete_database(
            &self,
            request: tonic::Request<super::DeleteDatabaseRequest>,
        ) -> Result<tonic::Response<super::DeleteDatabaseResponse>, tonic::Status>;
        async fn restore_database(
            &self,
            request: tonic::Request<super::RestoreDatabaseRequest>,
        ) -> Result<tonic::Response<super::RestoreDatabaseResponse>, tonic::Status>;
        #[doc = " List deleted databases and their metadata."]
        async fn list_deleted_databases(
            &self,
            request: tonic::Request<super::ListDeletedDatabasesRequest>,
        ) -> Result<tonic::Response<super::ListDeletedDatabasesResponse>, tonic::Status>;
        #[doc = " List all databases and their metadata."]
        async fn list_detailed_databases(
            &self,
            request: tonic::Request<super::ListDetailedDatabasesRequest>,
        ) -> Result<tonic::Response<super::ListDetailedDatabasesResponse>, tonic::Status>;
        #[doc = " List chunks available on this database"]
        async fn list_chunks(
            &self,
            request: tonic::Request<super::ListChunksRequest>,
        ) -> Result<tonic::Response<super::ListChunksResponse>, tonic::Status>;
        #[doc = " List remote IOx servers we know about."]
        async fn list_remotes(
            &self,
            request: tonic::Request<super::ListRemotesRequest>,
        ) -> Result<tonic::Response<super::ListRemotesResponse>, tonic::Status>;
        #[doc = " Update information about a remote IOx server (upsert)."]
        async fn update_remote(
            &self,
            request: tonic::Request<super::UpdateRemoteRequest>,
        ) -> Result<tonic::Response<super::UpdateRemoteResponse>, tonic::Status>;
        #[doc = " Delete a reference to remote IOx server."]
        async fn delete_remote(
            &self,
            request: tonic::Request<super::DeleteRemoteRequest>,
        ) -> Result<tonic::Response<super::DeleteRemoteResponse>, tonic::Status>;
        #[doc = " Creates a dummy job that for each value of the nanos field"]
        #[doc = " spawns a task that sleeps for that number of nanoseconds before returning"]
        async fn create_dummy_job(
            &self,
            request: tonic::Request<super::CreateDummyJobRequest>,
        ) -> Result<tonic::Response<super::CreateDummyJobResponse>, tonic::Status>;
        #[doc = " List partitions in a database"]
        async fn list_partitions(
            &self,
            request: tonic::Request<super::ListPartitionsRequest>,
        ) -> Result<tonic::Response<super::ListPartitionsResponse>, tonic::Status>;
        #[doc = " Get detail information about a partition"]
        async fn get_partition(
            &self,
            request: tonic::Request<super::GetPartitionRequest>,
        ) -> Result<tonic::Response<super::GetPartitionResponse>, tonic::Status>;
        #[doc = " List chunks in a partition"]
        async fn list_partition_chunks(
            &self,
            request: tonic::Request<super::ListPartitionChunksRequest>,
        ) -> Result<tonic::Response<super::ListPartitionChunksResponse>, tonic::Status>;
        #[doc = " Create a new chunk in the mutable buffer"]
        async fn new_partition_chunk(
            &self,
            request: tonic::Request<super::NewPartitionChunkRequest>,
        ) -> Result<tonic::Response<super::NewPartitionChunkResponse>, tonic::Status>;
        #[doc = " Close a chunk and move it to the read buffer"]
        async fn close_partition_chunk(
            &self,
            request: tonic::Request<super::ClosePartitionChunkRequest>,
        ) -> Result<tonic::Response<super::ClosePartitionChunkResponse>, tonic::Status>;
        #[doc = " Unload chunk from read buffer but keep it in object store"]
        async fn unload_partition_chunk(
            &self,
            request: tonic::Request<super::UnloadPartitionChunkRequest>,
        ) -> Result<tonic::Response<super::UnloadPartitionChunkResponse>, tonic::Status>;
        #[doc = " Get server status"]
        async fn get_server_status(
            &self,
            request: tonic::Request<super::GetServerStatusRequest>,
        ) -> Result<tonic::Response<super::GetServerStatusResponse>, tonic::Status>;
        #[doc = " Wipe preserved catalog for given DB."]
        async fn wipe_preserved_catalog(
            &self,
            request: tonic::Request<super::WipePreservedCatalogRequest>,
        ) -> Result<tonic::Response<super::WipePreservedCatalogResponse>, tonic::Status>;
        #[doc = " Skip replay for given DB."]
        async fn skip_replay(
            &self,
            request: tonic::Request<super::SkipReplayRequest>,
        ) -> Result<tonic::Response<super::SkipReplayResponse>, tonic::Status>;
        #[doc = " Persist given partition."]
        #[doc = ""]
        #[doc = " Errors if there is nothing to persist at the moment as per the lifecycle rules. If successful it returns the"]
        #[doc = " chunk that contains the persisted data."]
        async fn persist_partition(
            &self,
            request: tonic::Request<super::PersistPartitionRequest>,
        ) -> Result<tonic::Response<super::PersistPartitionResponse>, tonic::Status>;
        #[doc = " Drop partition from memory and (if persisted) from object store."]
        async fn drop_partition(
            &self,
            request: tonic::Request<super::DropPartitionRequest>,
        ) -> Result<tonic::Response<super::DropPartitionResponse>, tonic::Status>;
        #[doc = " Delete data for a table on a specified predicate"]
        async fn delete(
            &self,
            request: tonic::Request<super::DeleteRequest>,
        ) -> Result<tonic::Response<super::DeleteResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct ManagementServiceServer<T: ManagementService> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ManagementService> ManagementServiceServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ManagementServiceServer<T>
    where
        T: ManagementService,
        B: Body + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/influxdata.iox.management.v1.ManagementService/GetServerId" => {
                    #[allow(non_camel_case_types)]
                    struct GetServerIdSvc<T: ManagementService>(pub Arc<T>);
                    impl<T: ManagementService>
                        tonic::server::UnaryService<super::GetServerIdRequest>
                        for GetServerIdSvc<T>
                    {
                        type Response = super::GetServerIdResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetServerIdRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_server_id(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetServerIdSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/influxdata.iox.management.v1.ManagementService/UpdateServerId" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateServerIdSvc<T: ManagementService>(pub Arc<T>);
                    impl<T: ManagementService>
                        tonic::server::UnaryService<super::UpdateServerIdRequest>
                        for UpdateServerIdSvc<T>
                    {
                        type Response = super::UpdateServerIdResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateServerIdRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).update_server_id(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateServerIdSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/influxdata.iox.management.v1.ManagementService/SetServingReadiness" => {
                    #[allow(non_camel_case_types)]
                    struct SetServingReadinessSvc<T: ManagementService>(pub Arc<T>);
                    impl<T: ManagementService>
                        tonic::server::UnaryService<super::SetServingReadinessRequest>
                        for SetServingReadinessSvc<T>
                    {
                        type Response = super::SetServingReadinessResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetServingReadinessRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).set_serving_readiness(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetServingReadinessSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/influxdata.iox.management.v1.ManagementService/ListDatabases" => {
                    #[allow(non_camel_case_types)]
                    struct ListDatabasesSvc<T: ManagementService>(pub Arc<T>);
                    impl<T: ManagementService>
                        tonic::server::UnaryService<super::ListDatabasesRequest>
                        for ListDatabasesSvc<T>
                    {
                        type Response = super::ListDatabasesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListDatabasesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_databases(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListDatabasesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/influxdata.iox.management.v1.ManagementService/GetDatabase" => {
                    #[allow(non_camel_case_types)]
                    struct GetDatabaseSvc<T: ManagementService>(pub Arc<T>);
                    impl<T: ManagementService>
                        tonic::server::UnaryService<super::GetDatabaseRequest>
                        for GetDatabaseSvc<T>
                    {
                        type Response = super::GetDatabaseResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetDatabaseRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_database(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetDatabaseSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/influxdata.iox.management.v1.ManagementService/CreateDatabase" => {
                    #[allow(non_camel_case_types)]
                    struct CreateDatabaseSvc<T: ManagementService>(pub Arc<T>);
                    impl<T: ManagementService>
                        tonic::server::UnaryService<super::CreateDatabaseRequest>
                        for CreateDatabaseSvc<T>
                    {
                        type Response = super::CreateDatabaseResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateDatabaseRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_database(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateDatabaseSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/influxdata.iox.management.v1.ManagementService/UpdateDatabase" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateDatabaseSvc<T: ManagementService>(pub Arc<T>);
                    impl<T: ManagementService>
                        tonic::server::UnaryService<super::UpdateDatabaseRequest>
                        for UpdateDatabaseSvc<T>
                    {
                        type Response = super::UpdateDatabaseResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateDatabaseRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).update_database(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateDatabaseSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/influxdata.iox.management.v1.ManagementService/DeleteDatabase" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteDatabaseSvc<T: ManagementService>(pub Arc<T>);
                    impl<T: ManagementService>
                        tonic::server::UnaryService<super::DeleteDatabaseRequest>
                        for DeleteDatabaseSvc<T>
                    {
                        type Response = super::DeleteDatabaseResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteDatabaseRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete_database(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteDatabaseSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/influxdata.iox.management.v1.ManagementService/RestoreDatabase" => {
                    #[allow(non_camel_case_types)]
                    struct RestoreDatabaseSvc<T: ManagementService>(pub Arc<T>);
                    impl<T: ManagementService>
                        tonic::server::UnaryService<super::RestoreDatabaseRequest>
                        for RestoreDatabaseSvc<T>
                    {
                        type Response = super::RestoreDatabaseResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RestoreDatabaseRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).restore_database(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RestoreDatabaseSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/influxdata.iox.management.v1.ManagementService/ListDeletedDatabases" => {
                    #[allow(non_camel_case_types)]
                    struct ListDeletedDatabasesSvc<T: ManagementService>(pub Arc<T>);
                    impl<T: ManagementService>
                        tonic::server::UnaryService<super::ListDeletedDatabasesRequest>
                        for ListDeletedDatabasesSvc<T>
                    {
                        type Response = super::ListDeletedDatabasesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListDeletedDatabasesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_deleted_databases(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListDeletedDatabasesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/influxdata.iox.management.v1.ManagementService/ListDetailedDatabases" => {
                    #[allow(non_camel_case_types)]
                    struct ListDetailedDatabasesSvc<T: ManagementService>(pub Arc<T>);
                    impl<T: ManagementService>
                        tonic::server::UnaryService<super::ListDetailedDatabasesRequest>
                        for ListDetailedDatabasesSvc<T>
                    {
                        type Response = super::ListDetailedDatabasesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListDetailedDatabasesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).list_detailed_databases(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListDetailedDatabasesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/influxdata.iox.management.v1.ManagementService/ListChunks" => {
                    #[allow(non_camel_case_types)]
                    struct ListChunksSvc<T: ManagementService>(pub Arc<T>);
                    impl<T: ManagementService> tonic::server::UnaryService<super::ListChunksRequest>
                        for ListChunksSvc<T>
                    {
                        type Response = super::ListChunksResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListChunksRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_chunks(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListChunksSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/influxdata.iox.management.v1.ManagementService/ListRemotes" => {
                    #[allow(non_camel_case_types)]
                    struct ListRemotesSvc<T: ManagementService>(pub Arc<T>);
                    impl<T: ManagementService>
                        tonic::server::UnaryService<super::ListRemotesRequest>
                        for ListRemotesSvc<T>
                    {
                        type Response = super::ListRemotesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListRemotesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_remotes(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListRemotesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/influxdata.iox.management.v1.ManagementService/UpdateRemote" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateRemoteSvc<T: ManagementService>(pub Arc<T>);
                    impl<T: ManagementService>
                        tonic::server::UnaryService<super::UpdateRemoteRequest>
                        for UpdateRemoteSvc<T>
                    {
                        type Response = super::UpdateRemoteResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateRemoteRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).update_remote(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateRemoteSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/influxdata.iox.management.v1.ManagementService/DeleteRemote" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteRemoteSvc<T: ManagementService>(pub Arc<T>);
                    impl<T: ManagementService>
                        tonic::server::UnaryService<super::DeleteRemoteRequest>
                        for DeleteRemoteSvc<T>
                    {
                        type Response = super::DeleteRemoteResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteRemoteRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete_remote(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteRemoteSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/influxdata.iox.management.v1.ManagementService/CreateDummyJob" => {
                    #[allow(non_camel_case_types)]
                    struct CreateDummyJobSvc<T: ManagementService>(pub Arc<T>);
                    impl<T: ManagementService>
                        tonic::server::UnaryService<super::CreateDummyJobRequest>
                        for CreateDummyJobSvc<T>
                    {
                        type Response = super::CreateDummyJobResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateDummyJobRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_dummy_job(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateDummyJobSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/influxdata.iox.management.v1.ManagementService/ListPartitions" => {
                    #[allow(non_camel_case_types)]
                    struct ListPartitionsSvc<T: ManagementService>(pub Arc<T>);
                    impl<T: ManagementService>
                        tonic::server::UnaryService<super::ListPartitionsRequest>
                        for ListPartitionsSvc<T>
                    {
                        type Response = super::ListPartitionsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListPartitionsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_partitions(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListPartitionsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/influxdata.iox.management.v1.ManagementService/GetPartition" => {
                    #[allow(non_camel_case_types)]
                    struct GetPartitionSvc<T: ManagementService>(pub Arc<T>);
                    impl<T: ManagementService>
                        tonic::server::UnaryService<super::GetPartitionRequest>
                        for GetPartitionSvc<T>
                    {
                        type Response = super::GetPartitionResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetPartitionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_partition(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetPartitionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/influxdata.iox.management.v1.ManagementService/ListPartitionChunks" => {
                    #[allow(non_camel_case_types)]
                    struct ListPartitionChunksSvc<T: ManagementService>(pub Arc<T>);
                    impl<T: ManagementService>
                        tonic::server::UnaryService<super::ListPartitionChunksRequest>
                        for ListPartitionChunksSvc<T>
                    {
                        type Response = super::ListPartitionChunksResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListPartitionChunksRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_partition_chunks(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListPartitionChunksSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/influxdata.iox.management.v1.ManagementService/NewPartitionChunk" => {
                    #[allow(non_camel_case_types)]
                    struct NewPartitionChunkSvc<T: ManagementService>(pub Arc<T>);
                    impl<T: ManagementService>
                        tonic::server::UnaryService<super::NewPartitionChunkRequest>
                        for NewPartitionChunkSvc<T>
                    {
                        type Response = super::NewPartitionChunkResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NewPartitionChunkRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).new_partition_chunk(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewPartitionChunkSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/influxdata.iox.management.v1.ManagementService/ClosePartitionChunk" => {
                    #[allow(non_camel_case_types)]
                    struct ClosePartitionChunkSvc<T: ManagementService>(pub Arc<T>);
                    impl<T: ManagementService>
                        tonic::server::UnaryService<super::ClosePartitionChunkRequest>
                        for ClosePartitionChunkSvc<T>
                    {
                        type Response = super::ClosePartitionChunkResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ClosePartitionChunkRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).close_partition_chunk(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ClosePartitionChunkSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/influxdata.iox.management.v1.ManagementService/UnloadPartitionChunk" => {
                    #[allow(non_camel_case_types)]
                    struct UnloadPartitionChunkSvc<T: ManagementService>(pub Arc<T>);
                    impl<T: ManagementService>
                        tonic::server::UnaryService<super::UnloadPartitionChunkRequest>
                        for UnloadPartitionChunkSvc<T>
                    {
                        type Response = super::UnloadPartitionChunkResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UnloadPartitionChunkRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).unload_partition_chunk(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UnloadPartitionChunkSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/influxdata.iox.management.v1.ManagementService/GetServerStatus" => {
                    #[allow(non_camel_case_types)]
                    struct GetServerStatusSvc<T: ManagementService>(pub Arc<T>);
                    impl<T: ManagementService>
                        tonic::server::UnaryService<super::GetServerStatusRequest>
                        for GetServerStatusSvc<T>
                    {
                        type Response = super::GetServerStatusResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetServerStatusRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_server_status(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetServerStatusSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/influxdata.iox.management.v1.ManagementService/WipePreservedCatalog" => {
                    #[allow(non_camel_case_types)]
                    struct WipePreservedCatalogSvc<T: ManagementService>(pub Arc<T>);
                    impl<T: ManagementService>
                        tonic::server::UnaryService<super::WipePreservedCatalogRequest>
                        for WipePreservedCatalogSvc<T>
                    {
                        type Response = super::WipePreservedCatalogResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::WipePreservedCatalogRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).wipe_preserved_catalog(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = WipePreservedCatalogSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/influxdata.iox.management.v1.ManagementService/SkipReplay" => {
                    #[allow(non_camel_case_types)]
                    struct SkipReplaySvc<T: ManagementService>(pub Arc<T>);
                    impl<T: ManagementService> tonic::server::UnaryService<super::SkipReplayRequest>
                        for SkipReplaySvc<T>
                    {
                        type Response = super::SkipReplayResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SkipReplayRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).skip_replay(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SkipReplaySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/influxdata.iox.management.v1.ManagementService/PersistPartition" => {
                    #[allow(non_camel_case_types)]
                    struct PersistPartitionSvc<T: ManagementService>(pub Arc<T>);
                    impl<T: ManagementService>
                        tonic::server::UnaryService<super::PersistPartitionRequest>
                        for PersistPartitionSvc<T>
                    {
                        type Response = super::PersistPartitionResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PersistPartitionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).persist_partition(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PersistPartitionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/influxdata.iox.management.v1.ManagementService/DropPartition" => {
                    #[allow(non_camel_case_types)]
                    struct DropPartitionSvc<T: ManagementService>(pub Arc<T>);
                    impl<T: ManagementService>
                        tonic::server::UnaryService<super::DropPartitionRequest>
                        for DropPartitionSvc<T>
                    {
                        type Response = super::DropPartitionResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DropPartitionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).drop_partition(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DropPartitionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/influxdata.iox.management.v1.ManagementService/Delete" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteSvc<T: ManagementService>(pub Arc<T>);
                    impl<T: ManagementService> tonic::server::UnaryService<super::DeleteRequest> for DeleteSvc<T> {
                        type Response = super::DeleteResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: ManagementService> Clone for ManagementServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: ManagementService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ManagementService> tonic::transport::NamedService for ManagementServiceServer<T> {
        const NAME: &'static str = "influxdata.iox.management.v1.ManagementService";
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadata {
    /// How many nanoseconds of CPU time have been spent on this job so far?
    #[prost(uint64, tag = "1")]
    pub cpu_nanos: u64,
    /// How many nanoseconds has it been since the job was submitted
    #[prost(uint64, tag = "2")]
    pub wall_nanos: u64,
    /// The total number of created tasks
    #[prost(uint64, tag = "3")]
    pub total_count: u64,
    /// The number of pending tasks
    #[prost(uint64, tag = "4")]
    pub pending_count: u64,
    /// The number of tasks that completed successfully
    #[prost(uint64, tag = "13")]
    pub success_count: u64,
    /// The number of tasks that returned an error
    #[prost(uint64, tag = "14")]
    pub error_count: u64,
    /// The number of tasks that were cancelled
    #[prost(uint64, tag = "15")]
    pub cancelled_count: u64,
    /// The number of tasks that did not run to completion (e.g. panic)
    #[prost(uint64, tag = "16")]
    pub dropped_count: u64,
    /// What kind of job is it?
    #[prost(oneof = "operation_metadata::Job", tags = "5, 8, 9, 10, 11, 12, 17")]
    pub job: ::core::option::Option<operation_metadata::Job>,
}
/// Nested message and enum types in `OperationMetadata`.
pub mod operation_metadata {
    /// What kind of job is it?
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Job {
        #[prost(message, tag = "5")]
        Dummy(super::Dummy),
        #[prost(message, tag = "8")]
        WriteChunk(super::WriteChunk),
        #[prost(message, tag = "9")]
        WipePreservedCatalog(super::WipePreservedCatalog),
        #[prost(message, tag = "10")]
        CompactChunks(super::CompactChunks),
        #[prost(message, tag = "11")]
        PersistChunks(super::PersistChunks),
        #[prost(message, tag = "12")]
        DropChunk(super::DropChunk),
        #[prost(message, tag = "17")]
        DropPartition(super::DropPartition),
    }
}
/// A job that simply sleeps for a specified time and then returns success
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Dummy {
    /// How long the job should sleep for before returning
    #[prost(uint64, repeated, tag = "1")]
    pub nanos: ::prost::alloc::vec::Vec<u64>,
    /// Name of the database, if any
    #[prost(string, tag = "2")]
    pub db_name: ::prost::alloc::string::String,
}
/// Write a chunk from read buffer to object store
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteChunk {
    /// name of the database
    #[prost(string, tag = "1")]
    pub db_name: ::prost::alloc::string::String,
    /// partition key
    #[prost(string, tag = "2")]
    pub partition_key: ::prost::alloc::string::String,
    /// table name
    #[prost(string, tag = "4")]
    pub table_name: ::prost::alloc::string::String,
    /// chunk_id
    #[prost(uint32, tag = "3")]
    pub chunk_id: u32,
}
/// Compact chunks into a single chunk
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompactChunks {
    /// name of the database
    #[prost(string, tag = "1")]
    pub db_name: ::prost::alloc::string::String,
    /// partition key
    #[prost(string, tag = "2")]
    pub partition_key: ::prost::alloc::string::String,
    /// table name
    #[prost(string, tag = "4")]
    pub table_name: ::prost::alloc::string::String,
    /// chunk_id
    #[prost(uint32, repeated, tag = "3")]
    pub chunks: ::prost::alloc::vec::Vec<u32>,
}
/// Split and write chunks to object store
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PersistChunks {
    /// name of the database
    #[prost(string, tag = "1")]
    pub db_name: ::prost::alloc::string::String,
    /// partition key
    #[prost(string, tag = "2")]
    pub partition_key: ::prost::alloc::string::String,
    /// table name
    #[prost(string, tag = "4")]
    pub table_name: ::prost::alloc::string::String,
    /// chunk_id
    #[prost(uint32, repeated, tag = "3")]
    pub chunks: ::prost::alloc::vec::Vec<u32>,
}
/// Drop chunk from memory and (if persisted) from object store.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DropChunk {
    /// name of the database
    #[prost(string, tag = "1")]
    pub db_name: ::prost::alloc::string::String,
    /// partition key
    #[prost(string, tag = "2")]
    pub partition_key: ::prost::alloc::string::String,
    /// table name
    #[prost(string, tag = "4")]
    pub table_name: ::prost::alloc::string::String,
    /// chunk_id
    #[prost(uint32, tag = "3")]
    pub chunk_id: u32,
}
/// Drop partition from memory and (if persisted) from object store.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DropPartition {
    /// name of the database
    #[prost(string, tag = "1")]
    pub db_name: ::prost::alloc::string::String,
    /// partition key
    #[prost(string, tag = "2")]
    pub partition_key: ::prost::alloc::string::String,
    /// table name
    #[prost(string, tag = "4")]
    pub table_name: ::prost::alloc::string::String,
}
/// Wipe preserved catalog
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WipePreservedCatalog {
    /// name of the database
    #[prost(string, tag = "1")]
    pub db_name: ::prost::alloc::string::String,
}
/// Soft delete data from a table on a specified predicate
/// Soft delete means data is deleted from customer's point of view but
/// they still physically exist in IOx storage (MUB, RUB, OS). During Query time,
/// we will filter out the deleted rows.
/// We will implement Hard Delete to purge deleted data.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Delete {
    /// name of the database
    #[prost(string, tag = "1")]
    pub db_name: ::prost::alloc::string::String,
    /// table name
    #[prost(string, tag = "2")]
    pub table_name: ::prost::alloc::string::String,
    /// delete predicate
    /// Ideally, this can be any complicated expressions that DataFusion supports
    /// but in our first version, we only support what our read buffer does which is
    /// conjunctive expressions with columns being compared to literals using = or != operators.
    /// Also, to avoid user from making mistake to delete the whole table, we will force them to
    /// include delete time range start and stop in different fields defined below
    #[prost(string, tag = "3")]
    pub delete_predicate: ::prost::alloc::string::String,
    /// start time range of deleting data
    #[prost(string, tag = "4")]
    pub start_time: ::prost::alloc::string::String,
    /// stop time range of deleting data
    #[prost(string, tag = "5")]
    pub stop_time: ::prost::alloc::string::String,
}
