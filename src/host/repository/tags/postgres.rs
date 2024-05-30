use sqlx::postgres::PgRow;
use sqlx::Row;

use crate::host::repository::{error::QueryError, postgres::PostgresDatabase};

use super::TagsRepository;

impl TagsRepository for PostgresDatabase {
    async fn list(&self, project_id: &str) -> Result<Vec<String>, QueryError> {
        const LIST_QUERY: &'static str = r#"
            SELECT tag
            FROM project_tags
            WHERE project_id = $1
        "#;

        let mut conn = self.connection_pool
            .get()
            .await?;

        sqlx::query(LIST_QUERY)
            .bind(project_id)
            .map(|row: PgRow| row.get("tag"))
            .fetch_all(conn.as_mut())
            .await
            .map_err(QueryError::from)
    }

    async fn create(&self, project_id: &str, tag: &str) -> Result<(), QueryError> {
        const INSERT_QUERY: &'static str = r#"
            INSERT INTO project_tags (project_id, tag)
            VALUES ($1, $2)
        "#;

        let mut conn = self.connection_pool
            .get()
            .await?;

        sqlx::query(INSERT_QUERY)
            .bind(project_id)
            .bind(tag)
            .execute(conn.as_mut())
            .await
            .map_err(QueryError::from)?;

        Ok(())
    }

    async fn delete(&self, project_id: &str, tag: &str) -> Result<(), QueryError> {
        const DELETE_QUERY: &'static str = r#"
            DELETE FROM project_tags
            WHERE project_id = $1 AND tag = $2
        "#;

        let mut conn = self.connection_pool
            .get()
            .await?;

        sqlx::query(DELETE_QUERY)
            .bind(project_id)
            .bind(tag)
            .execute(conn.as_mut())
            .await
            .map_err(QueryError::from)?;

        Ok(())
    }
}