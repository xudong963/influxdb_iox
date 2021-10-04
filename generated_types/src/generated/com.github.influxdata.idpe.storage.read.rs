#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadSource {
    /// OrgID specifies the organization identifier for this request.
    #[prost(uint64, tag = "1")]
    pub org_id: u64,
    /// BucketID specifies the bucket in the organization.
    #[prost(uint64, tag = "2")]
    pub bucket_id: u64,
    /// PartitionID specifies the partition to be queried.
    #[prost(uint64, tag = "3")]
    pub partition_id: u64,
}
