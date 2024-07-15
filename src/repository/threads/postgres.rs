use sqlx::postgres::PgRow;
use sqlx::Row;

use crate::repository::{error::QueryError, postgres::PostgresDatabase, threads::Thread};

use super::{Comment, NewComment, NewThread, ThreadsRepository};

impl ThreadsRepository for PostgresDatabase {
    async fn create<'a>(&self, thread: NewThread<'a>) -> Result<(), QueryError> {
        const INSERT_QUERY: &'static str = r#"
            INSERT INTO project_threads (id, project_id, user_id, title)
            VALUES ($1, $2, $3, $4)
        "#;

        let mut conn = self.connection_pool
            .get()
            .await?;

        sqlx::query(INSERT_QUERY)
            .bind(thread.id)
            .bind(thread.project_id)
            .bind(thread.user_id)
            .bind(thread.title)
            .fetch_one(conn.as_mut())
            .await
            .map(|row| row.get("id"))
            .map_err(QueryError::from)
    }

    async fn list(&self, project_id: &str) -> Result<Vec<Thread>, QueryError> {
        const LIST_QUERY: &'static str = r#"
            SELECT id, project_id, user_id, title
            FROM project_threads
            WHERE project_id = $1
        "#;

        let mut conn = self.connection_pool
            .get()
            .await?;

        sqlx::query(LIST_QUERY)
            .bind(project_id)
            .map(|row: PgRow| Thread {
                id: row.get("id"),
                project_id: row.get("project_id"),
                user_id: row.get("user_id"),
                title: row.get("title"),
            })
            .fetch_all(conn.as_mut())
            .await
            .map_err(QueryError::from)
    }

    async fn get_by_id<'a>(&self, id: &'a str) -> Result<Option<Thread>, QueryError> {
        const THREAD_QUERY: &'static str = r#"
            SELECT id, project_id, user_id, title
            FROM project_threads
            WHERE id = $1
        "#;

        let mut conn = self.connection_pool
            .get()
            .await?;

        sqlx::query(THREAD_QUERY)
            .bind(id)
            .map(|row: PgRow| Thread {
                id: row.get("id"),
                project_id: row.get("project_id"),
                user_id: row.get("user_id"),
                title: row.get("title"),
            })
            .fetch_optional(conn.as_mut())
            .await
            .map_err(QueryError::from)
    }   

    async fn create_comment<'a>(&self, comment: NewComment<'a>) -> Result<(), QueryError> {
        const INSERT_QUERY: &'static str = r#"
            INSERT INTO project_thread_comments (id, thread_id, user_id, content)
            VALUES ($1, $2, $3, $4, $5)
        "#;

        let mut conn = self.connection_pool
            .get()
            .await?;

        sqlx::query(INSERT_QUERY)
            .bind(comment.id)
            .bind(comment.thread_id)
            .bind(comment.user_id)
            .bind(comment.content)
            .fetch_one(conn.as_mut())
            .await
            .map(|row| row.get("id"))
            .map_err(QueryError::from)
    }

    async fn list_comments<'a>(&self, thread_id: &'a str) -> Result<Vec<Comment>, QueryError> {
        const LIST_QUERY: &'static str = r#"
            SELECT id, thread_id, user_id, content, created_at
            FROM project_thread_comments
            WHERE thread_id = $1
        "#;

        let mut conn = self.connection_pool
            .get()
            .await?;

        sqlx::query(LIST_QUERY)
            .bind(thread_id)
            .map(|row: PgRow| Comment {
                id: row.get("id"),
                thread_id: row.get("thread_id"),
                user_id: row.get("user_id"),
                content: row.get("content"),
            })
            .fetch_all(conn.as_mut())
            .await
            .map_err(QueryError::from)
    }
}