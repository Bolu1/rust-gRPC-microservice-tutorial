use chrono::prelude::*;
use uuid::Uuid;

use crate::models::enums::project::CarbonProjectCategory;

pub struct TaskInSchema{
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub is_completed: bool,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
}