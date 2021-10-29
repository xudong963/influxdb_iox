use thiserror::Error;

use self::generated_types::{dummy_job_service_client::DummyJobServiceClient, *};

use crate::connection::Connection;

use crate::google::{longrunning::IoxOperation, FieldViolation};
use std::convert::TryInto;

/// Re-export generated_types
pub mod generated_types {
    pub use generated_types::influxdata::iox::dummy_job::v1::*;
}

/// Errors returned by Client::create_dummy_job
#[derive(Debug, Error)]
pub enum CreateDummyJobError {
    /// Response contained no payload
    #[error("Server returned an empty response")]
    EmptyResponse,

    /// Response payload was invalid
    #[error("Invalid response: {0}")]
    InvalidResponse(#[from] FieldViolation),

    /// Client received an unexpected error from the server
    #[error("Unexpected server error: {}: {}", .0.code(), .0.message())]
    ServerError(tonic::Status),
}

/// An IOx Dummy Job API client.
///
/// This client wraps the underlying `tonic` generated client with a
/// more ergonomic interface.
///
/// ```no_run
/// #[tokio::main]
/// # async fn main() {
/// use influxdb_iox_client::{
///     management::Client,
///     connection::Builder,
/// };
///
/// let mut connection = Builder::default()
///     .build("http://127.0.0.1:8082")
///     .await
///     .unwrap();
///
/// let mut client = Client::new(connection);
///
/// // Create a new database!
/// client
///     .create_dummy_job(vec![1_000])
///     .await
///     .expect("failed to create database");
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct Client {
    inner: DummyJobServiceClient<Connection>,
}

impl Client {
    /// Creates a new client with the provided connection
    pub fn new(channel: Connection) -> Self {
        Self {
            inner: DummyJobServiceClient::new(channel),
        }
    }
    /// Creates a dummy job that for each value of the nanos field
    /// spawns a task that sleeps for that number of nanoseconds before
    /// returning
    pub async fn create_dummy_job(
        &mut self,
        nanos: Vec<u64>,
    ) -> Result<IoxOperation, CreateDummyJobError> {
        let response = self
            .inner
            .create_dummy_job(CreateDummyJobRequest { nanos })
            .await
            .map_err(CreateDummyJobError::ServerError)?;

        Ok(response
            .into_inner()
            .operation
            .ok_or(CreateDummyJobError::EmptyResponse)?
            .try_into()?)
    }
}
