use grpc_service::task_service::TasksInGrpc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, LocalResult, TimeZone, Utc};

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct TaskInHttp {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    #[serde(rename = "isCompleted")]
    pub is_completed: bool,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct CreateTaskHttpPayload {
    pub title: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct CreateTaskHttpResponse {
    pub status: String,
    pub data: CreateTaskHttpResponseData,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct CreateTaskHttpResponseData {
    #[serde(rename = "taskId")]
    pub task_id: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct UpdateTaskHttpUrlParameter {
    pub task_id: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct UpdateTaskHttpPayload {
    #[serde(rename = "isCompleted")]
    pub is_completed: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct UpdateTaskHttpResponse {
    pub status: String,
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct ReadTasksHttpQuery {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct ReadTasksHttpResponse {
    pub status: String,
    pub data: ReadTasksHttpResponseData,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct ReadTasksHttpResponseData {
    pub tasks: Vec<TaskInHttp>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct DeleteTaskHttpUrlParameter {
    pub task_id: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct DeleteTaskHttpResponse {
    pub status: String,
    pub message: String,
}

impl TaskInHttp {
    pub fn into_grpc(task: &TasksInGrpc) -> TaskInHttp {
        TaskInHttp {
            id: Uuid::parse_str(task.id.as_str()).unwrap_or_default(),
            title: task.title.to_owned(),
            description: Some(task.description.to_owned()),
            is_completed: task.is_completed,
            created_at: TaskInHttp::convert_timestamp_to_utc(task.created_at),
            updated_at: TaskInHttp::convert_timestamp_to_utc(task.updated_at)
        }
    }

    pub fn tasks_into_grpc(tasks: Vec<TasksInGrpc>) -> Vec<TaskInHttp> {
        tasks.iter().map(TaskInHttp::into_grpc).collect()
    }

    pub fn convert_timestamp_to_utc(timestamp: i64) -> Option<DateTime<Utc>> {
        match Utc.timestamp_opt(timestamp, 0) {
            LocalResult::Single(datetime) => Some(datetime),
            _ => None, // Handle invalid timestamp
        }
    }
}


