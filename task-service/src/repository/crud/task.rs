use async_trait::async_trait;
use sqlx::Error;
use uuid::Uuid;

use crate::models::domain::task::TaskInDB;
use crate::models::schemas::task::TaskInCreate;
use crate::repository::db::Database;

#[async_trait]
pub trait TaskExt {
    async fn create_task<
        T: Into<String> + Send,
        U: Into<Option<String>> + Send,
    >(
        &self,
        title: T,
        description: U,
    ) -> Result<TaskInCreate, Error>;

    async fn read_tasks(
        &self,
        page: u32,
        limit: usize,
    ) -> Result<Vec<TaskInDB>, Error>;
    async fn update_task_status<
            T: Into<Uuid> + Send,
            U: Into<bool> + Send,
        >(
        &self,
        task_id: T,
        is_completed: U,
    ) -> Result<(), Error>;
    async fn delete_tasks<T: Into<Uuid> + Send>(&self, task_id: T) -> Result<(), Error>;
}

#[async_trait]
impl TaskExt for Database {
    async fn create_task<
        T: Into<String> + Send,
        U: Into<Option<String>> + Send,
    >(
        &self,
        title: T,
        description: U,
    ) -> Result<TaskInCreate, Error> {
        sqlx::query_as!(
            TaskInCreate,
            r#"INSERT INTO tasks (title, description)
            VALUES ($1, $2) RETURNING id"#,
            title.into(),
            description.into(),
        )
        .fetch_one(&self.pool)
        .await
    }

    async fn read_tasks(
        &self,
        page: u32,
        limit: usize,
    ) -> Result<Vec<TaskInDB>, Error> {
        let offset = (page - 1) * limit as u32;
        sqlx::query_as!(
                TaskInDB,
                r#"SELECT *
                FROM tasks
                ORDER BY created_at DESC LIMIT $1 OFFSET $2"#,
                limit as i64,
                offset as i64,
            )
            .fetch_all(&self.pool)
            .await
        }

    async fn update_task_status<
            T: Into<Uuid> + Send,
            U: Into<bool> + Send,
        >(
        &self,
        task_id: T,
        is_completed: U,
    ) -> Result<(), Error> {
        sqlx::query_as!(
            None,
            r#"UPDATE tasks SET is_completed = $2 WHERE id = $1"#,
            task_id.into(),
            is_completed.into(),
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete_tasks<T: Into<Uuid> + Send>(&self, task_id: T) -> Result<(), Error> {
        sqlx::query_as!(
            None,
            r#"DELETE FROM tasks WHERE id = $1"#,
            task_id.into()
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
