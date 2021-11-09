use std::sync::Arc;

use async_trait::async_trait;
use hyper::{Body, Request, Response, StatusCode};
use metric::Registry;
use snafu::Snafu;
use trace::TraceCollector;

use super::rpc::RpcBuilderInput;

pub mod common_state;
pub mod database;
pub mod router;

/// Constants used in API error codes.
///
/// Expressing this as a enum prevents reuse of discriminants, and as they're
/// effectively consts this uses UPPER_SNAKE_CASE.
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq)]
pub enum ApiErrorCode {
    /// An unknown/unhandled error
    UNKNOWN = 100,

    /// The database name in the request is invalid.
    DB_INVALID_NAME = 101,

    /// The database referenced already exists.
    DB_ALREADY_EXISTS = 102,

    /// The database referenced does not exist.
    DB_NOT_FOUND = 103,
}

impl From<ApiErrorCode> for u32 {
    fn from(v: ApiErrorCode) -> Self {
        v as Self
    }
}

pub trait RouteError: std::error::Error + snafu::AsErrorSource {
    fn response(&self) -> Response<Body>;

    fn bad_request(&self, api_error_code: ApiErrorCode) -> Response<Body> {
        Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(self.body(api_error_code))
            .unwrap()
    }

    fn internal_error(&self, api_error_code: ApiErrorCode) -> Response<Body> {
        Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(self.body(api_error_code))
            .unwrap()
    }

    fn not_found(&self) -> Response<Body> {
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::empty())
            .unwrap()
    }

    fn no_content(&self, api_error_code: ApiErrorCode) -> Response<Body> {
        Response::builder()
            .status(StatusCode::NO_CONTENT)
            .body(self.body(api_error_code))
            .unwrap()
    }

    fn body(&self, api_error_code: ApiErrorCode) -> Body {
        let api_error_code: u32 = api_error_code.into();

        let json = serde_json::json!({
            "error": self.to_string(),
            "error_code": api_error_code,
        })
        .to_string();

        Body::from(json)
    }

    /// Checks if this should be treated as an internal server error.
    fn is_internal(&self) -> bool {
        self.response().status().is_server_error()
    }
}

#[derive(Debug, Snafu)]
pub enum RpcError {
    #[snafu(display("gRPC transport error: {}{}", source, details))]
    TransportError {
        source: tonic::transport::Error,
        details: String,
    },
}

// Custom impl to include underlying source (not included in tonic
// transport error)
impl From<tonic::transport::Error> for RpcError {
    fn from(source: tonic::transport::Error) -> Self {
        use std::error::Error;
        let details = source
            .source()
            .map(|e| format!(" ({})", e))
            .unwrap_or_else(|| "".to_string());

        Self::TransportError { source, details }
    }
}

#[async_trait]
pub trait ServerType: std::fmt::Debug + Send + Sync + 'static {
    type RouteError: RouteError;

    fn metric_registry(&self) -> Arc<Registry>;

    fn trace_collector(&self) -> Option<Arc<dyn TraceCollector>>;

    /// Route given HTTP request.
    ///
    /// Note that this is only called if none of the shared, common routes (e.g. `/health`) match.
    async fn route_http_request(
        &self,
        req: Request<Body>,
    ) -> Result<Response<Body>, Self::RouteError>;

    async fn server_grpc(self: Arc<Self>, builder_input: RpcBuilderInput) -> Result<(), RpcError>;

    async fn background_worker(self: Arc<Self>);

    fn shutdown_background_worker(&self);
}
