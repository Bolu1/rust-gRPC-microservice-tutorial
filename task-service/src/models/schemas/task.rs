use chrono::prelude::*;
use uuid::Uuid;

pub struct TaskInSchema{
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub is_completed: bool,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
}

pub struct TaskInCreate{
    pub id: Uuid,
}