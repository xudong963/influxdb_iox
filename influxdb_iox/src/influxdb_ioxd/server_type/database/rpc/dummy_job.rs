use std::sync::Arc;

use generated_types::influxdata::iox::dummy_job::v1::*;
use server::JobRegistry;
use tonic::{Request, Response, Status};

struct DummyJobService {
    jobs: Arc<JobRegistry>,
}

#[tonic::async_trait]
impl dummy_job_service_server::DummyJobService for DummyJobService {
    async fn create_dummy_job(
        &self,
        request: Request<CreateDummyJobRequest>,
    ) -> Result<Response<CreateDummyJobResponse>, Status> {
        let request = request.into_inner();
        let tracker = self.jobs.spawn_dummy_job(request.nanos, None);
        let operation = Some(super::operations::encode_tracker(tracker)?);
        Ok(Response::new(CreateDummyJobResponse { operation }))
    }
}

pub fn make_server(
    jobs: Arc<JobRegistry>,
) -> dummy_job_service_server::DummyJobServiceServer<impl dummy_job_service_server::DummyJobService>
{
    dummy_job_service_server::DummyJobServiceServer::new(DummyJobService { jobs })
}
