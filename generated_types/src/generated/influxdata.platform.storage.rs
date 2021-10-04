/// Send a message that will generate an internal error (used for testing)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestErrorRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestErrorResponse {}
#[doc = r" Generated client implementations."]
pub mod i_ox_testing_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct IOxTestingClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl IOxTestingClient<tonic::transport::Channel> {
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
    impl<T> IOxTestingClient<T>
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
        ) -> IOxTestingClient<InterceptedService<T, F>>
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
            IOxTestingClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn test_error(
            &mut self,
            request: impl tonic::IntoRequest<super::TestErrorRequest>,
        ) -> Result<tonic::Response<super::TestErrorResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/influxdata.platform.storage.IOxTesting/TestError",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod i_ox_testing_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with IOxTestingServer."]
    #[async_trait]
    pub trait IOxTesting: Send + Sync + 'static {
        async fn test_error(
            &self,
            request: tonic::Request<super::TestErrorRequest>,
        ) -> Result<tonic::Response<super::TestErrorResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct IOxTestingServer<T: IOxTesting> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: IOxTesting> IOxTestingServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for IOxTestingServer<T>
    where
        T: IOxTesting,
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
                "/influxdata.platform.storage.IOxTesting/TestError" => {
                    #[allow(non_camel_case_types)]
                    struct TestErrorSvc<T: IOxTesting>(pub Arc<T>);
                    impl<T: IOxTesting> tonic::server::UnaryService<super::TestErrorRequest> for TestErrorSvc<T> {
                        type Response = super::TestErrorResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TestErrorRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).test_error(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TestErrorSvc(inner);
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
    impl<T: IOxTesting> Clone for IOxTestingServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: IOxTesting> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: IOxTesting> tonic::transport::NamedService for IOxTestingServer<T> {
        const NAME: &'static str = "influxdata.platform.storage.IOxTesting";
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Node {
    #[prost(enumeration = "node::Type", tag = "1")]
    pub node_type: i32,
    #[prost(message, repeated, tag = "2")]
    pub children: ::prost::alloc::vec::Vec<Node>,
    #[prost(oneof = "node::Value", tags = "3, 4, 5, 6, 7, 8, 9, 10, 11, 12")]
    pub value: ::core::option::Option<node::Value>,
}
/// Nested message and enum types in `Node`.
pub mod node {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        LogicalExpression = 0,
        ComparisonExpression = 1,
        ParenExpression = 2,
        TagRef = 3,
        Literal = 4,
        FieldRef = 5,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Comparison {
        Equal = 0,
        NotEqual = 1,
        StartsWith = 2,
        Regex = 3,
        NotRegex = 4,
        Lt = 5,
        Lte = 6,
        Gt = 7,
        Gte = 8,
    }
    /// Logical operators apply to boolean values and combine to produce a single boolean result.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Logical {
        And = 0,
        Or = 1,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        #[prost(string, tag = "3")]
        StringValue(::prost::alloc::string::String),
        #[prost(bool, tag = "4")]
        BoolValue(bool),
        #[prost(int64, tag = "5")]
        IntValue(i64),
        #[prost(uint64, tag = "6")]
        UintValue(u64),
        #[prost(double, tag = "7")]
        FloatValue(f64),
        #[prost(string, tag = "8")]
        RegexValue(::prost::alloc::string::String),
        ///    string tag_ref_value = 9;
        /// AAL changed from string --> bytes to handle \xff literals in Rust which are not valid UTF-8
        #[prost(bytes, tag = "9")]
        TagRefValue(::prost::alloc::vec::Vec<u8>),
        #[prost(string, tag = "10")]
        FieldRefValue(::prost::alloc::string::String),
        #[prost(enumeration = "Logical", tag = "11")]
        Logical(i32),
        #[prost(enumeration = "Comparison", tag = "12")]
        Comparison(i32),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Predicate {
    #[prost(message, optional, tag = "1")]
    pub root: ::core::option::Option<Node>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadFilterRequest {
    #[prost(message, optional, tag = "1")]
    pub read_source: ::core::option::Option<::pbjson_types::Any>,
    #[prost(message, optional, tag = "2")]
    pub range: ::core::option::Option<TimestampRange>,
    #[prost(message, optional, tag = "3")]
    pub predicate: ::core::option::Option<Predicate>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadGroupRequest {
    #[prost(message, optional, tag = "1")]
    pub read_source: ::core::option::Option<::pbjson_types::Any>,
    #[prost(message, optional, tag = "2")]
    pub range: ::core::option::Option<TimestampRange>,
    #[prost(message, optional, tag = "3")]
    pub predicate: ::core::option::Option<Predicate>,
    /// GroupKeys specifies a list of tag keys used to order the data.
    /// It is dependent on the Group property to determine its behavior.
    #[prost(string, repeated, tag = "4")]
    pub group_keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(enumeration = "read_group_request::Group", tag = "5")]
    pub group: i32,
    #[prost(message, optional, tag = "6")]
    pub aggregate: ::core::option::Option<Aggregate>,
    #[prost(fixed32, tag = "7")]
    pub hints: u32,
}
/// Nested message and enum types in `ReadGroupRequest`.
pub mod read_group_request {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Group {
        /// GroupNone returns all series as a single group.
        /// The single GroupFrame.TagKeys will be the union of all tag keys.
        None = 0,
        /// GroupBy returns a group for each unique value of the specified GroupKeys.
        By = 2,
    }
    /// TODO(jlapacik): This field is only used in unit tests.
    /// Specifically the two tests in group_resultset_test.go.
    /// This field should be removed and the tests that depend
    /// on it refactored.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum HintFlags {
        HintNone = 0,
        HintNoPoints = 1,
        HintNoSeries = 2,
        /// HintSchemaAllTime performs schema queries without using time ranges
        HintSchemaAllTime = 4,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Aggregate {
    #[prost(enumeration = "aggregate::AggregateType", tag = "1")]
    pub r#type: i32,
}
/// Nested message and enum types in `Aggregate`.
pub mod aggregate {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AggregateType {
        None = 0,
        Sum = 1,
        Count = 2,
        Min = 3,
        Max = 4,
        First = 5,
        Last = 6,
        Mean = 7,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tag {
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
/// Response message for ReadFilter and ReadGroup
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadResponse {
    #[prost(message, repeated, tag = "1")]
    pub frames: ::prost::alloc::vec::Vec<read_response::Frame>,
}
/// Nested message and enum types in `ReadResponse`.
pub mod read_response {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Frame {
        #[prost(oneof = "frame::Data", tags = "7, 1, 2, 3, 4, 5, 6")]
        pub data: ::core::option::Option<frame::Data>,
    }
    /// Nested message and enum types in `Frame`.
    pub mod frame {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Data {
            #[prost(message, tag = "7")]
            Group(super::GroupFrame),
            #[prost(message, tag = "1")]
            Series(super::SeriesFrame),
            #[prost(message, tag = "2")]
            FloatPoints(super::FloatPointsFrame),
            #[prost(message, tag = "3")]
            IntegerPoints(super::IntegerPointsFrame),
            #[prost(message, tag = "4")]
            UnsignedPoints(super::UnsignedPointsFrame),
            #[prost(message, tag = "5")]
            BooleanPoints(super::BooleanPointsFrame),
            #[prost(message, tag = "6")]
            StringPoints(super::StringPointsFrame),
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GroupFrame {
        /// TagKeys
        #[prost(bytes = "vec", repeated, tag = "1")]
        pub tag_keys: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
        /// PartitionKeyVals is the values of the partition key for this group, order matching ReadGroupRequest.GroupKeys
        #[prost(bytes = "vec", repeated, tag = "2")]
        pub partition_key_vals: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SeriesFrame {
        #[prost(message, repeated, tag = "1")]
        pub tags: ::prost::alloc::vec::Vec<super::Tag>,
        #[prost(enumeration = "DataType", tag = "2")]
        pub data_type: i32,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FloatPointsFrame {
        #[prost(sfixed64, repeated, tag = "1")]
        pub timestamps: ::prost::alloc::vec::Vec<i64>,
        #[prost(double, repeated, tag = "2")]
        pub values: ::prost::alloc::vec::Vec<f64>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct IntegerPointsFrame {
        #[prost(sfixed64, repeated, tag = "1")]
        pub timestamps: ::prost::alloc::vec::Vec<i64>,
        #[prost(int64, repeated, tag = "2")]
        pub values: ::prost::alloc::vec::Vec<i64>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UnsignedPointsFrame {
        #[prost(sfixed64, repeated, tag = "1")]
        pub timestamps: ::prost::alloc::vec::Vec<i64>,
        #[prost(uint64, repeated, tag = "2")]
        pub values: ::prost::alloc::vec::Vec<u64>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BooleanPointsFrame {
        #[prost(sfixed64, repeated, tag = "1")]
        pub timestamps: ::prost::alloc::vec::Vec<i64>,
        #[prost(bool, repeated, tag = "2")]
        pub values: ::prost::alloc::vec::Vec<bool>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct StringPointsFrame {
        #[prost(sfixed64, repeated, tag = "1")]
        pub timestamps: ::prost::alloc::vec::Vec<i64>,
        #[prost(string, repeated, tag = "2")]
        pub values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum FrameType {
        Series = 0,
        Points = 1,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum DataType {
        Float = 0,
        Integer = 1,
        Unsigned = 2,
        Boolean = 3,
        String = 4,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Capability {
    /// Features contains the specific features supported
    /// by this capability.
    #[prost(string, repeated, tag = "1")]
    pub features: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CapabilitiesResponse {
    /// Capabilities contains the set of capabilities supported
    /// by the storage engine. It is a map of method names to
    /// the detailed capability information for the method.
    #[prost(map = "string, message", tag = "1")]
    pub caps: ::std::collections::HashMap<::prost::alloc::string::String, Capability>,
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
/// TagKeysRequest is the request message for Storage.TagKeys.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TagKeysRequest {
    #[prost(message, optional, tag = "1")]
    pub tags_source: ::core::option::Option<::pbjson_types::Any>,
    #[prost(message, optional, tag = "2")]
    pub range: ::core::option::Option<TimestampRange>,
    #[prost(message, optional, tag = "3")]
    pub predicate: ::core::option::Option<Predicate>,
}
/// TagValuesRequest is the request message for Storage.TagValues.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TagValuesRequest {
    #[prost(message, optional, tag = "1")]
    pub tags_source: ::core::option::Option<::pbjson_types::Any>,
    /// [(gogoproto.nullable) = false];
    #[prost(message, optional, tag = "2")]
    pub range: ::core::option::Option<TimestampRange>,
    #[prost(message, optional, tag = "3")]
    pub predicate: ::core::option::Option<Predicate>,
    /// string tag_key = 4;
    /// AAL changed from string --> bytes to handle \xff literals in Rust which are not valid UTF-8
    #[prost(bytes = "vec", tag = "4")]
    pub tag_key: ::prost::alloc::vec::Vec<u8>,
}
/// Response message for Storage.TagKeys, Storage.TagValues Storage.MeasurementNames,
/// Storage.MeasurementTagKeys and Storage.MeasurementTagValues.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StringValuesResponse {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub values: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// MeasurementNamesRequest is the request message for Storage.MeasurementNames.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MeasurementNamesRequest {
    #[prost(message, optional, tag = "1")]
    pub source: ::core::option::Option<::pbjson_types::Any>,
    /// [(gogoproto.nullable) = false]
    #[prost(message, optional, tag = "2")]
    pub range: ::core::option::Option<TimestampRange>,
    #[prost(message, optional, tag = "3")]
    pub predicate: ::core::option::Option<Predicate>,
}
/// MeasurementTagKeysRequest is the request message for Storage.MeasurementTagKeys.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MeasurementTagKeysRequest {
    #[prost(message, optional, tag = "1")]
    pub source: ::core::option::Option<::pbjson_types::Any>,
    #[prost(string, tag = "2")]
    pub measurement: ::prost::alloc::string::String,
    /// [(gogoproto.nullable) = false]
    #[prost(message, optional, tag = "3")]
    pub range: ::core::option::Option<TimestampRange>,
    #[prost(message, optional, tag = "4")]
    pub predicate: ::core::option::Option<Predicate>,
}
/// MeasurementTagValuesRequest is the request message for Storage.MeasurementTagValues.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MeasurementTagValuesRequest {
    #[prost(message, optional, tag = "1")]
    pub source: ::core::option::Option<::pbjson_types::Any>,
    #[prost(string, tag = "2")]
    pub measurement: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub tag_key: ::prost::alloc::string::String,
    /// [(gogoproto.nullable) = false];
    #[prost(message, optional, tag = "4")]
    pub range: ::core::option::Option<TimestampRange>,
    #[prost(message, optional, tag = "5")]
    pub predicate: ::core::option::Option<Predicate>,
}
/// MeasurementFieldsRequest is the request message for Storage.MeasurementFields.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MeasurementFieldsRequest {
    #[prost(message, optional, tag = "1")]
    pub source: ::core::option::Option<::pbjson_types::Any>,
    #[prost(string, tag = "2")]
    pub measurement: ::prost::alloc::string::String,
    /// [(gogoproto.nullable) = false];
    #[prost(message, optional, tag = "3")]
    pub range: ::core::option::Option<TimestampRange>,
    #[prost(message, optional, tag = "4")]
    pub predicate: ::core::option::Option<Predicate>,
}
/// MeasurementFieldsResponse is the response message for Storage.MeasurementFields.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MeasurementFieldsResponse {
    /// [(gogoproto.nullable) = false];
    #[prost(message, repeated, tag = "1")]
    pub fields: ::prost::alloc::vec::Vec<measurement_fields_response::MessageField>,
}
/// Nested message and enum types in `MeasurementFieldsResponse`.
pub mod measurement_fields_response {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MessageField {
        #[prost(string, tag = "1")]
        pub key: ::prost::alloc::string::String,
        #[prost(enumeration = "FieldType", tag = "2")]
        pub r#type: i32,
        #[prost(sfixed64, tag = "3")]
        pub timestamp: i64,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum FieldType {
        Float = 0,
        Integer = 1,
        Unsigned = 2,
        String = 3,
        Boolean = 4,
        Undefined = 5,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadWindowAggregateRequest {
    #[prost(message, optional, tag = "1")]
    pub read_source: ::core::option::Option<::pbjson_types::Any>,
    /// [(gogoproto.nullable) = false];
    #[prost(message, optional, tag = "2")]
    pub range: ::core::option::Option<TimestampRange>,
    #[prost(message, optional, tag = "3")]
    pub predicate: ::core::option::Option<Predicate>,
    #[prost(int64, tag = "4")]
    pub window_every: i64,
    #[prost(int64, tag = "6")]
    pub offset: i64,
    #[prost(message, repeated, tag = "5")]
    pub aggregate: ::prost::alloc::vec::Vec<Aggregate>,
    #[prost(message, optional, tag = "7")]
    pub window: ::core::option::Option<Window>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Window {
    #[prost(message, optional, tag = "1")]
    pub every: ::core::option::Option<Duration>,
    #[prost(message, optional, tag = "2")]
    pub offset: ::core::option::Option<Duration>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Duration {
    #[prost(int64, tag = "1")]
    pub nsecs: i64,
    #[prost(int64, tag = "2")]
    pub months: i64,
    #[prost(bool, tag = "3")]
    pub negative: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadSeriesCardinalityRequest {
    #[prost(message, optional, tag = "1")]
    pub read_series_cardinality_source: ::core::option::Option<::pbjson_types::Any>,
    /// [(gogoproto.nullable) = false];
    #[prost(message, optional, tag = "2")]
    pub range: ::core::option::Option<TimestampRange>,
    #[prost(message, optional, tag = "3")]
    pub predicate: ::core::option::Option<Predicate>,
}
/// Response message for Storage.SeriesCardinality
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Int64ValuesResponse {
    #[prost(int64, repeated, tag = "1")]
    pub values: ::prost::alloc::vec::Vec<i64>,
}
#[doc = r" Generated client implementations."]
pub mod storage_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct StorageClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl StorageClient<tonic::transport::Channel> {
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
    impl<T> StorageClient<T>
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
        ) -> StorageClient<InterceptedService<T, F>>
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
            StorageClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " ReadFilter performs a filter operation at storage"]
        pub async fn read_filter(
            &mut self,
            request: impl tonic::IntoRequest<super::ReadFilterRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::ReadResponse>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/influxdata.platform.storage.Storage/ReadFilter",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " ReadGroup performs a group operation at storage"]
        pub async fn read_group(
            &mut self,
            request: impl tonic::IntoRequest<super::ReadGroupRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::ReadResponse>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/influxdata.platform.storage.Storage/ReadGroup",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " ReadWindowAggregate performs a window aggregate operation at storage"]
        pub async fn read_window_aggregate(
            &mut self,
            request: impl tonic::IntoRequest<super::ReadWindowAggregateRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::ReadResponse>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/influxdata.platform.storage.Storage/ReadWindowAggregate",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " TagKeys performs a read operation for tag keys"]
        pub async fn tag_keys(
            &mut self,
            request: impl tonic::IntoRequest<super::TagKeysRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::StringValuesResponse>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/influxdata.platform.storage.Storage/TagKeys",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " TagValues performs a read operation for tag values"]
        pub async fn tag_values(
            &mut self,
            request: impl tonic::IntoRequest<super::TagValuesRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::StringValuesResponse>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/influxdata.platform.storage.Storage/TagValues",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " ReadSeriesCardinality performs a read operation for series cardinality"]
        pub async fn read_series_cardinality(
            &mut self,
            request: impl tonic::IntoRequest<super::ReadSeriesCardinalityRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::Int64ValuesResponse>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/influxdata.platform.storage.Storage/ReadSeriesCardinality",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " Capabilities returns a map of keys and values identifying the capabilities supported by the storage engine"]
        pub async fn capabilities(
            &mut self,
            request: impl tonic::IntoRequest<::pbjson_types::Empty>,
        ) -> Result<tonic::Response<super::CapabilitiesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/influxdata.platform.storage.Storage/Capabilities",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn measurement_names(
            &mut self,
            request: impl tonic::IntoRequest<super::MeasurementNamesRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::StringValuesResponse>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/influxdata.platform.storage.Storage/MeasurementNames",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        pub async fn measurement_tag_keys(
            &mut self,
            request: impl tonic::IntoRequest<super::MeasurementTagKeysRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::StringValuesResponse>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/influxdata.platform.storage.Storage/MeasurementTagKeys",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        pub async fn measurement_tag_values(
            &mut self,
            request: impl tonic::IntoRequest<super::MeasurementTagValuesRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::StringValuesResponse>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/influxdata.platform.storage.Storage/MeasurementTagValues",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        pub async fn measurement_fields(
            &mut self,
            request: impl tonic::IntoRequest<super::MeasurementFieldsRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::MeasurementFieldsResponse>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/influxdata.platform.storage.Storage/MeasurementFields",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod storage_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with StorageServer."]
    #[async_trait]
    pub trait Storage: Send + Sync + 'static {
        #[doc = "Server streaming response type for the ReadFilter method."]
        type ReadFilterStream: futures_core::Stream<Item = Result<super::ReadResponse, tonic::Status>>
            + Send
            + Sync
            + 'static;
        #[doc = " ReadFilter performs a filter operation at storage"]
        async fn read_filter(
            &self,
            request: tonic::Request<super::ReadFilterRequest>,
        ) -> Result<tonic::Response<Self::ReadFilterStream>, tonic::Status>;
        #[doc = "Server streaming response type for the ReadGroup method."]
        type ReadGroupStream: futures_core::Stream<Item = Result<super::ReadResponse, tonic::Status>>
            + Send
            + Sync
            + 'static;
        #[doc = " ReadGroup performs a group operation at storage"]
        async fn read_group(
            &self,
            request: tonic::Request<super::ReadGroupRequest>,
        ) -> Result<tonic::Response<Self::ReadGroupStream>, tonic::Status>;
        #[doc = "Server streaming response type for the ReadWindowAggregate method."]
        type ReadWindowAggregateStream: futures_core::Stream<Item = Result<super::ReadResponse, tonic::Status>>
            + Send
            + Sync
            + 'static;
        #[doc = " ReadWindowAggregate performs a window aggregate operation at storage"]
        async fn read_window_aggregate(
            &self,
            request: tonic::Request<super::ReadWindowAggregateRequest>,
        ) -> Result<tonic::Response<Self::ReadWindowAggregateStream>, tonic::Status>;
        #[doc = "Server streaming response type for the TagKeys method."]
        type TagKeysStream: futures_core::Stream<Item = Result<super::StringValuesResponse, tonic::Status>>
            + Send
            + Sync
            + 'static;
        #[doc = " TagKeys performs a read operation for tag keys"]
        async fn tag_keys(
            &self,
            request: tonic::Request<super::TagKeysRequest>,
        ) -> Result<tonic::Response<Self::TagKeysStream>, tonic::Status>;
        #[doc = "Server streaming response type for the TagValues method."]
        type TagValuesStream: futures_core::Stream<Item = Result<super::StringValuesResponse, tonic::Status>>
            + Send
            + Sync
            + 'static;
        #[doc = " TagValues performs a read operation for tag values"]
        async fn tag_values(
            &self,
            request: tonic::Request<super::TagValuesRequest>,
        ) -> Result<tonic::Response<Self::TagValuesStream>, tonic::Status>;
        #[doc = "Server streaming response type for the ReadSeriesCardinality method."]
        type ReadSeriesCardinalityStream: futures_core::Stream<Item = Result<super::Int64ValuesResponse, tonic::Status>>
            + Send
            + Sync
            + 'static;
        #[doc = " ReadSeriesCardinality performs a read operation for series cardinality"]
        async fn read_series_cardinality(
            &self,
            request: tonic::Request<super::ReadSeriesCardinalityRequest>,
        ) -> Result<tonic::Response<Self::ReadSeriesCardinalityStream>, tonic::Status>;
        #[doc = " Capabilities returns a map of keys and values identifying the capabilities supported by the storage engine"]
        async fn capabilities(
            &self,
            request: tonic::Request<::pbjson_types::Empty>,
        ) -> Result<tonic::Response<super::CapabilitiesResponse>, tonic::Status>;
        #[doc = "Server streaming response type for the MeasurementNames method."]
        type MeasurementNamesStream: futures_core::Stream<Item = Result<super::StringValuesResponse, tonic::Status>>
            + Send
            + Sync
            + 'static;
        async fn measurement_names(
            &self,
            request: tonic::Request<super::MeasurementNamesRequest>,
        ) -> Result<tonic::Response<Self::MeasurementNamesStream>, tonic::Status>;
        #[doc = "Server streaming response type for the MeasurementTagKeys method."]
        type MeasurementTagKeysStream: futures_core::Stream<Item = Result<super::StringValuesResponse, tonic::Status>>
            + Send
            + Sync
            + 'static;
        async fn measurement_tag_keys(
            &self,
            request: tonic::Request<super::MeasurementTagKeysRequest>,
        ) -> Result<tonic::Response<Self::MeasurementTagKeysStream>, tonic::Status>;
        #[doc = "Server streaming response type for the MeasurementTagValues method."]
        type MeasurementTagValuesStream: futures_core::Stream<Item = Result<super::StringValuesResponse, tonic::Status>>
            + Send
            + Sync
            + 'static;
        async fn measurement_tag_values(
            &self,
            request: tonic::Request<super::MeasurementTagValuesRequest>,
        ) -> Result<tonic::Response<Self::MeasurementTagValuesStream>, tonic::Status>;
        #[doc = "Server streaming response type for the MeasurementFields method."]
        type MeasurementFieldsStream: futures_core::Stream<Item = Result<super::MeasurementFieldsResponse, tonic::Status>>
            + Send
            + Sync
            + 'static;
        async fn measurement_fields(
            &self,
            request: tonic::Request<super::MeasurementFieldsRequest>,
        ) -> Result<tonic::Response<Self::MeasurementFieldsStream>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct StorageServer<T: Storage> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Storage> StorageServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for StorageServer<T>
    where
        T: Storage,
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
                "/influxdata.platform.storage.Storage/ReadFilter" => {
                    #[allow(non_camel_case_types)]
                    struct ReadFilterSvc<T: Storage>(pub Arc<T>);
                    impl<T: Storage> tonic::server::ServerStreamingService<super::ReadFilterRequest>
                        for ReadFilterSvc<T>
                    {
                        type Response = super::ReadResponse;
                        type ResponseStream = T::ReadFilterStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ReadFilterRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).read_filter(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ReadFilterSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/influxdata.platform.storage.Storage/ReadGroup" => {
                    #[allow(non_camel_case_types)]
                    struct ReadGroupSvc<T: Storage>(pub Arc<T>);
                    impl<T: Storage> tonic::server::ServerStreamingService<super::ReadGroupRequest>
                        for ReadGroupSvc<T>
                    {
                        type Response = super::ReadResponse;
                        type ResponseStream = T::ReadGroupStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ReadGroupRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).read_group(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ReadGroupSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/influxdata.platform.storage.Storage/ReadWindowAggregate" => {
                    #[allow(non_camel_case_types)]
                    struct ReadWindowAggregateSvc<T: Storage>(pub Arc<T>);
                    impl<T: Storage>
                        tonic::server::ServerStreamingService<super::ReadWindowAggregateRequest>
                        for ReadWindowAggregateSvc<T>
                    {
                        type Response = super::ReadResponse;
                        type ResponseStream = T::ReadWindowAggregateStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ReadWindowAggregateRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).read_window_aggregate(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ReadWindowAggregateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/influxdata.platform.storage.Storage/TagKeys" => {
                    #[allow(non_camel_case_types)]
                    struct TagKeysSvc<T: Storage>(pub Arc<T>);
                    impl<T: Storage> tonic::server::ServerStreamingService<super::TagKeysRequest> for TagKeysSvc<T> {
                        type Response = super::StringValuesResponse;
                        type ResponseStream = T::TagKeysStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TagKeysRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).tag_keys(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TagKeysSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/influxdata.platform.storage.Storage/TagValues" => {
                    #[allow(non_camel_case_types)]
                    struct TagValuesSvc<T: Storage>(pub Arc<T>);
                    impl<T: Storage> tonic::server::ServerStreamingService<super::TagValuesRequest>
                        for TagValuesSvc<T>
                    {
                        type Response = super::StringValuesResponse;
                        type ResponseStream = T::TagValuesStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TagValuesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).tag_values(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TagValuesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/influxdata.platform.storage.Storage/ReadSeriesCardinality" => {
                    #[allow(non_camel_case_types)]
                    struct ReadSeriesCardinalitySvc<T: Storage>(pub Arc<T>);
                    impl<T: Storage>
                        tonic::server::ServerStreamingService<super::ReadSeriesCardinalityRequest>
                        for ReadSeriesCardinalitySvc<T>
                    {
                        type Response = super::Int64ValuesResponse;
                        type ResponseStream = T::ReadSeriesCardinalityStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ReadSeriesCardinalityRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).read_series_cardinality(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ReadSeriesCardinalitySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/influxdata.platform.storage.Storage/Capabilities" => {
                    #[allow(non_camel_case_types)]
                    struct CapabilitiesSvc<T: Storage>(pub Arc<T>);
                    impl<T: Storage> tonic::server::UnaryService<::pbjson_types::Empty> for CapabilitiesSvc<T> {
                        type Response = super::CapabilitiesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<::pbjson_types::Empty>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).capabilities(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CapabilitiesSvc(inner);
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
                "/influxdata.platform.storage.Storage/MeasurementNames" => {
                    #[allow(non_camel_case_types)]
                    struct MeasurementNamesSvc<T: Storage>(pub Arc<T>);
                    impl<T: Storage>
                        tonic::server::ServerStreamingService<super::MeasurementNamesRequest>
                        for MeasurementNamesSvc<T>
                    {
                        type Response = super::StringValuesResponse;
                        type ResponseStream = T::MeasurementNamesStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MeasurementNamesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).measurement_names(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MeasurementNamesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/influxdata.platform.storage.Storage/MeasurementTagKeys" => {
                    #[allow(non_camel_case_types)]
                    struct MeasurementTagKeysSvc<T: Storage>(pub Arc<T>);
                    impl<T: Storage>
                        tonic::server::ServerStreamingService<super::MeasurementTagKeysRequest>
                        for MeasurementTagKeysSvc<T>
                    {
                        type Response = super::StringValuesResponse;
                        type ResponseStream = T::MeasurementTagKeysStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MeasurementTagKeysRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).measurement_tag_keys(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MeasurementTagKeysSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/influxdata.platform.storage.Storage/MeasurementTagValues" => {
                    #[allow(non_camel_case_types)]
                    struct MeasurementTagValuesSvc<T: Storage>(pub Arc<T>);
                    impl<T: Storage>
                        tonic::server::ServerStreamingService<super::MeasurementTagValuesRequest>
                        for MeasurementTagValuesSvc<T>
                    {
                        type Response = super::StringValuesResponse;
                        type ResponseStream = T::MeasurementTagValuesStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MeasurementTagValuesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).measurement_tag_values(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MeasurementTagValuesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/influxdata.platform.storage.Storage/MeasurementFields" => {
                    #[allow(non_camel_case_types)]
                    struct MeasurementFieldsSvc<T: Storage>(pub Arc<T>);
                    impl<T: Storage>
                        tonic::server::ServerStreamingService<super::MeasurementFieldsRequest>
                        for MeasurementFieldsSvc<T>
                    {
                        type Response = super::MeasurementFieldsResponse;
                        type ResponseStream = T::MeasurementFieldsStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MeasurementFieldsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).measurement_fields(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MeasurementFieldsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
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
    impl<T: Storage> Clone for StorageServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Storage> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Storage> tonic::transport::NamedService for StorageServer<T> {
        const NAME: &'static str = "influxdata.platform.storage.Storage";
    }
}
