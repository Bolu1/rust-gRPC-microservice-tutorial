use crate::models::schemas::task::{
    CreateTaskHttpPayload, CreateTaskHttpResponse, CreateTaskHttpResponseData, DeleteTaskHttpResponse, DeleteTaskHttpUrlParameter, ReadTasksHttpQuery, ReadTasksHttpResponse, ReadTasksHttpResponseData, TaskInHttp, UpdateTaskHttpPayload, UpdateTaskHttpResponse, UpdateTaskHttpUrlParameter
    };
use actix_web::{web, Error, HttpResponse, Scope};
use grpc_service::task_service::{
    CreateTaskGrpcPayload, 
    UpdateTaskGrpcPayload,
    ReadTasksGrpcPayload,
    DeleteTaskGrpcPayload,
};

use crate::models::schemas::app::AppState;

pub fn task_scope() -> Scope {
    web::scope("/project")
        .route(
            "",
            web::post()
                .to(create_task_project)
        )
        .route(
            "",
            web::get()
                .to(read_task_projects)
        )
        .route(
            "/{task_id}",
            web::patch()
                .to(update_task_project)
        )
        .route(
            "/{task_id}",
            web::delete()
                .to(delete_task_project)
        )
}

async fn create_task_project(
    app_state: web::Data<AppState>,
    payload: web::Json<CreateTaskHttpPayload>,
) -> Result<HttpResponse, Error> {
    let grpc_request_payload = CreateTaskGrpcPayload {
        title: payload.title.to_owned(),
        description: payload.description.to_owned().unwrap_or_default(),
    };

    let response = app_state
        .service_client
        .task_service_client
        .to_owned()
        .unwrap()
        .create_task(grpc_request_payload)
        .await
        .expect("Error creating task");

    let response = response.into_inner();

    Ok(HttpResponse::Ok().json(CreateTaskHttpResponse {
        status: "success".to_string(),
        data: CreateTaskHttpResponseData {
            task_id: response.task_id,
        },
    }))
}

async fn read_task_projects(
    app_state: web::Data<AppState>,
    query: web::Query<ReadTasksHttpQuery>,
) -> Result<HttpResponse, Error> {
    let query_params: ReadTasksHttpQuery = query.into_inner();

    let page = query_params.page.unwrap_or(1);
    let limit = query_params.limit.unwrap_or(10);

    let grpc_request_payload = ReadTasksGrpcPayload {
        page: page as i32,
        limit: limit as i32,
    };

    let response = app_state
        .service_client
        .task_service_client
        .to_owned()
        .unwrap()
        .read_tasks(grpc_request_payload)
        .await
        .expect("Error creating task");

    let response = response.into_inner();

    Ok(HttpResponse::Ok().json(ReadTasksHttpResponse {
        status: "success".to_string(),
        data: ReadTasksHttpResponseData {
            tasks: TaskInHttp::tasks_into_grpc(response.tasks)
        },
    }))
}

async fn update_task_project(
    app_state: web::Data<AppState>,
    payload: web::Json<UpdateTaskHttpPayload>,
    params: web::Path<UpdateTaskHttpUrlParameter>
) -> Result<HttpResponse, Error> {
    let grpc_request_payload = UpdateTaskGrpcPayload {
        task_id: params.task_id.to_owned(),
        is_completed: payload.is_completed,
    };

    let _ = app_state
        .service_client
        .task_service_client
        .to_owned()
        .unwrap()
        .update_task(grpc_request_payload)
        .await
        .expect("Error creating task");

    Ok(HttpResponse::Ok().json(UpdateTaskHttpResponse {
        status: "success".to_string(),
        message: "Task updated sucesfully".to_string(),
    }))
}

async fn delete_task_project(
    app_state: web::Data<AppState>,
    params: web::Path<DeleteTaskHttpUrlParameter>,
) -> Result<HttpResponse, Error> {
    let grpc_request_payload = DeleteTaskGrpcPayload {
        task_id: params.task_id.to_owned(),
    };

    app_state
        .service_client
        .task_service_client
        .to_owned()
        .unwrap()
        .delete_task(grpc_request_payload)
        .await
        .expect("Error creating task");

    Ok(HttpResponse::Ok().json(DeleteTaskHttpResponse {
        status: "success".to_string(),
        message: "Project deleted successfully".to_string(),
    }))
}


