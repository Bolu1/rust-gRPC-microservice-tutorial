use crate::{models::domain::task::TaskInDB, repository::crud::task::TaskExt};
use crate::models::schemas::app::TaskServiceService;
use grpc_service::task_service::{
    task_service_server::TaskService, CreateTaskGrpcPayload, CreateTaskGrpcResponse,
    ReadTasksGrpcPayload, ReadTasksGrpcResponse,
    UpdateTaskGrpcPayload, UpdateTaskGrpcResponse,
    DeleteTaskGrpcPayload, DeleteTaskGrpcResponse
};
use tonic::{Code, Request, Response, Status};
use uuid::Uuid;

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
                payload.title.clone(),
                payload.description.clone()
            )
            .await
            .expect("Error saving task");

        Ok(Response::new(CreateTaskGrpcResponse {
            task_id: result.id.to_string()
        }))
    }

    async fn read_tasks(
        &self,
        request: Request<ReadTasksGrpcPayload>,
    ) -> Result<Response<ReadTasksGrpcResponse>, Status> {
        let payload: &ReadTasksGrpcPayload = request.get_ref();

        let tasks = self
            .app_state
            .db
            .read_tasks(
                payload.page as u32,
                payload.limit as usize,
            )
            .await
            .expect("Error reading tasks");

        Ok(Response::new(ReadTasksGrpcResponse {
            tasks: TaskInDB::tasks_into_grpc(tasks)
        }))
    }

    async fn update_task(
        &self,
        request: Request<UpdateTaskGrpcPayload>,
    ) -> Result<Response<UpdateTaskGrpcResponse>, Status> {
        let payload: &UpdateTaskGrpcPayload = request.get_ref();

        _ = self
            .app_state
            .db
            .update_task_status(
                Uuid::parse_str(&payload.task_id).expect("Error converting string into Uuid"),
                payload.is_completed
            )
            .await;

        Ok(Response::new(UpdateTaskGrpcResponse {
            message: "Task status updated successfully".to_string(),
        }))
    }

    async fn delete_task(
        &self,
        request: Request<DeleteTaskGrpcPayload>,
    ) -> Result<Response<DeleteTaskGrpcResponse>, Status> {
        let payload: &DeleteTaskGrpcPayload = request.get_ref();

        self
            .app_state
            .db
            .delete_tasks(
                Uuid::parse_str(&payload.task_id).expect("Error converting string into Uuid"),
            )
            .await
            .map_err(|_| Status::new(Code::NotFound, "Not found"))?;

        Ok(Response::new(DeleteTaskGrpcResponse {
            message: "Task deleted successfully".to_string(),
        }))
    }

}
