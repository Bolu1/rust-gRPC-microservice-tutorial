use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use grpc_service::task_service::TasksInGrpc;

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct TaskInDB {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub is_completed: bool,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
}

impl TaskInDB {
    pub fn into_grpc(&self) -> TasksInGrpc {
        TasksInGrpc {
            id: self.id.to_string(),
            title: self.title.to_owned(),
            description: self.description.to_owned().unwrap_or_default(),
            is_completed: self.is_completed,
            created_at: self.created_at.unwrap_or_default().timestamp(),
            updated_at: self.updated_at.unwrap_or_default().timestamp(),
        }
    }

    pub fn tasks_into_grpc(tasks: Vec<TaskInDB>) -> Vec<TasksInGrpc> {
        tasks.iter().map(TaskInDB::into_grpc).collect()
    }
    
}