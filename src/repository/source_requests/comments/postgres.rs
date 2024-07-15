use sqlx::postgres::PgRow;
use sqlx::Row;

use crate::repository::{error::QueryError, postgres::PostgresDatabase};

use super::{SourceRequestComment, SourceRequestCommentRepository};

impl SourceRequestCommentRepository for PostgresDatabase {
    async fn create<'a>(&self, source_request_comment: super::CreateSourceRequestComment<'a>) -> Result<i32, QueryError> {
        const CREATE_QUERY: &'static str = r#"
            INSERT INTO source_request_comments (source_request_id, user_id, content)
            VALUES ($1, $2, $3)
            RETURNING id
        "#;

        let mut conn = self.connection_pool
            .get()
            .await?;

        sqlx::query(CREATE_QUERY)
            .bind(source_request_comment.source_request_id)
            .bind(source_request_comment.user_id)
            .bind(source_request_comment.content)
            .map(|row: PgRow| row.get("id"))
            .fetch_one(conn.as_mut())
            .await
            .map_err(QueryError::from)
    }

    async fn list<'a>(&self, source_request_id: &'a str) -> Result<Vec<SourceRequestComment>, QueryError> {
        const LIST_QUERY: &'static str = r#"
            SELECT id, source_request_id, user_id, content, created_at
            FROM source_request_comments
            WHERE source_request_id = $1
        "#;

        let mut conn = self.connection_pool
            .get()
            .await?;

        sqlx::query(LIST_QUERY)
            .bind(source_request_id)
            .map(|row: PgRow| SourceRequestComment {
                id: row.get("id"),
                source_request_id: row.get("source_request_id"),
                user_id: row.get("user_id"),
                content: row.get("content"),
            })
            .fetch_all(conn.as_mut())
            .await
            .map_err(QueryError::from)
    }
}