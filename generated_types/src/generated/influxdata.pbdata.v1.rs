#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatabaseBatch {
    #[prost(string, tag = "1")]
    pub database_name: ::prost::alloc::string::String,
    /// Table data. Data for a given table may appear in multiple table batches.
    #[prost(message, repeated, tag = "2")]
    pub table_batches: ::prost::alloc::vec::Vec<TableBatch>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableBatch {
    #[prost(string, tag = "1")]
    pub table_name: ::prost::alloc::string::String,
    /// Data are represented here.
    ///
    /// Exactly one column named and typed "time" *must* exist,
    /// and *must not* contain null values.
    ///
    /// For line protocol tables (tables containing tags and fields),
    /// columns *should* be sorted by cardinality, from lowest to highest
    /// (the time column is always last in the sort order).
    #[prost(message, repeated, tag = "2")]
    pub columns: ::prost::alloc::vec::Vec<Column>,
    /// Length of all columns in this table batch.
    /// This count includes null and non-null values.
    ///
    /// All columns in a TableBatch must have equal length.
    #[prost(uint32, tag = "3")]
    pub row_count: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Column {
    #[prost(string, tag = "1")]
    pub column_name: ::prost::alloc::string::String,
    /// Semantic meaning behind the data.
    #[prost(enumeration = "column::SemanticType", tag = "2")]
    pub semantic_type: i32,
    /// The sequence of non-null values contained in this column.
    /// Order matters; positions are adjusted by null positions in null_mask.
    ///
    /// For example, column "foo", containing I64 values (10,11,12,13,14,null,16,17,null,99,100):
    ///   Column:
    ///     column_name: foo
    ///     values:
    ///       i64_values: 10,11,12,13,14,16,17,99,100
    ///                7      0  15     8
    ///     null_mask: 00100000  00000001
    ///
    /// Exactly one of the fields within the Values message *should* be set.
    #[prost(message, optional, tag = "3")]
    pub values: ::core::option::Option<column::Values>,
    /// Mask that maps the positions of null values.
    /// Null positions hold space between non-null values in the values field.
    ///
    /// An on bit (1) indicates that the column value at that position is null.
    /// If zero null values exist in the column, then null_mask *may* be omitted.
    /// If zero non-null values in the column, then the column *should* be omitted.
    /// Trailing off bits (0) *may* be omitted.
    #[prost(bytes = "vec", tag = "4")]
    pub null_mask: ::prost::alloc::vec::Vec<u8>,
}
/// Nested message and enum types in `Column`.
pub mod column {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Values {
        #[prost(int64, repeated, tag = "1")]
        pub i64_values: ::prost::alloc::vec::Vec<i64>,
        #[prost(double, repeated, tag = "2")]
        pub f64_values: ::prost::alloc::vec::Vec<f64>,
        #[prost(uint64, repeated, tag = "3")]
        pub u64_values: ::prost::alloc::vec::Vec<u64>,
        #[prost(string, repeated, tag = "4")]
        pub string_values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        #[prost(bool, repeated, tag = "5")]
        pub bool_values: ::prost::alloc::vec::Vec<bool>,
        #[prost(bytes = "vec", repeated, tag = "6")]
        pub bytes_values: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SemanticType {
        Unspecified = 0,
        /// "Native" semantic type (value types: i64, f64, u64, string, bool, bytes)
        Iox = 1,
        /// InfluxDB/TSM tag (value type string only)
        Tag = 2,
        /// InfluxDB/TSM field (value types: i64, f64, u64, string, bool)
        Field = 3,
        /// Timestamps, which must have value type i64
        Time = 4,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteRequest {
    #[prost(message, optional, tag = "1")]
    pub database_batch: ::core::option::Option<DatabaseBatch>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteResponse {}
#[doc = r" Generated client implementations."]
pub mod write_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct WriteServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl WriteServiceClient<tonic::transport::Channel> {
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
    impl<T> WriteServiceClient<T>
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
        ) -> WriteServiceClient<InterceptedService<T, F>>
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
            WriteServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn write(
            &mut self,
            request: impl tonic::IntoRequest<super::WriteRequest>,
        ) -> Result<tonic::Response<super::WriteResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/influxdata.pbdata.v1.WriteService/Write");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod write_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with WriteServiceServer."]
    #[async_trait]
    pub trait WriteService: Send + Sync + 'static {
        async fn write(
            &self,
            request: tonic::Request<super::WriteRequest>,
        ) -> Result<tonic::Response<super::WriteResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct WriteServiceServer<T: WriteService> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: WriteService> WriteServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for WriteServiceServer<T>
    where
        T: WriteService,
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
                "/influxdata.pbdata.v1.WriteService/Write" => {
                    #[allow(non_camel_case_types)]
                    struct WriteSvc<T: WriteService>(pub Arc<T>);
                    impl<T: WriteService> tonic::server::UnaryService<super::WriteRequest> for WriteSvc<T> {
                        type Response = super::WriteResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::WriteRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).write(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = WriteSvc(inner);
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
    impl<T: WriteService> Clone for WriteServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: WriteService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: WriteService> tonic::transport::NamedService for WriteServiceServer<T> {
        const NAME: &'static str = "influxdata.pbdata.v1.WriteService";
    }
}
