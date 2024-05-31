use crate::repository::crud::task::TaskExt;
use crate::models::schemas::app::TaskServiceService;
use grpc_service::task_service::{
    task_service_server::TaskService, CreateTaskGrpcPayload, CreateTaskGrpcResponse,
    ReadTasksGrpcPayload, ReadTasksGrpcResponse
};
use tonic::{Code, Request, Response, Status};

#[tonic::async_trait]
impl TaskService for TaskServiceService {
    async fn create_task(
        &self,
        request: Request<CreateTaskGrpcPayload>,
    ) -> Result<Response<CreateTaskGrpcResponse>, Status> {

        let payload: &CreateTaskGrpcPayload = request.get_ref();
        let result = self
            .app_state
            .db
            .create_task(
                payload.title,
                payload.description
            )
            .await
            .expect("Error saving carbon project");

        Ok(Response::new(CreateTaskGrpcResponse {
            task_id:result.id.to_string()
        }))
    }

    async fn read_tasks(
        &self,
        request: Request<ReadTasksGrpcPayload>,
    ) -> Result<Response<ReadTasksGrpcResponse>, Status> {
        let payload: &ReadTasksGrpcPayload = request.get_ref();

        let carbon_projects_in_read = self
            .app_state
            .db
            .read_tasks(
                payload.page as u32,
                payload.limit as usize,
            )
            .await
            .expect("Error reading carbon projects");

        Ok(Response::new(ReadTasksGrpcResponse {
            count: carbon_projects_in_read.count,
            carbon_projects: carbon_projects_in_read.carbon_projects,
        }))
    }

}