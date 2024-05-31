use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct TaskInDB {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub is_completed: bool,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
}