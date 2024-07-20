use sqlx::{postgres::PgRow, types::Json};
use sqlx::Row;

use crate::{events::Snapshot, repository::{error::QueryError, postgres::PostgresDatabase}};

use super::{ListQuery, SnapshotsRepository};

impl SnapshotsRepository for PostgresDatabase {
    async fn list(&self, query: &ListQuery) -> Result<Vec<Snapshot>, QueryError> {
        const SNAPSHOT_QUERY: &'static str = r#"
            SELECT content
            FROM project_snapshots
            WHERE 
                ($1 IS NULL OR project_id = UNNEST($1))
                AND ($2 IS NULL OR content->>'uid' = $2)
        "#;

        let mut conn = self.connection_pool
            .get()
            .await?;

        let query = match query {
            ListQuery::ProjectIds { project_ids } => sqlx::query(&SNAPSHOT_QUERY).bind(project_ids).bind(None::<String>),
            ListQuery::UserId { user_id } => sqlx::query(&SNAPSHOT_QUERY).bind(None::<Vec<String>>).bind(user_id),
        };

        query
            .map(|row: PgRow| row.get("content"))
            .map(|content: Json<Snapshot>| content.0)
            .fetch_all(conn.as_mut())
            .await
            .map_err(QueryError::from)
    }

    async fn create(&self, project_id: &str, version: &str, snapshot: impl Into<Snapshot>) -> Result<(), QueryError> {
        const INSERT_QUERY: &'static str = r#"
            INSERT INTO project_snapshots (project_id, version, content)
            VALUES ($1, $2, $3)
        "#;

        let mut conn = self.connection_pool
            .get()
            .await?;

        sqlx::query(INSERT_QUERY)
            .bind(project_id)
            .bind(version)
            .bind(Json::from(snapshot.into()))
            .execute(conn.as_mut())
            .await
            .map(|_| ())
            .map_err(QueryError::from)
    }

    async fn get_by_id(&self, project_id: &str, version: &str) -> Result<Option<Snapshot>, QueryError> {
        const SNAPSHOT_QUERY: &'static str = r#"
            SELECT content
            FROM project_snapshots
            WHERE project_id = $1 AND version = $2
        "#;

        let mut conn = self.connection_pool
            .get()
            .await?;

        sqlx::query(&SNAPSHOT_QUERY)
            .bind(project_id)
            .bind(version)
            .map(|row: PgRow| row.get("content"))
            .map(|content: Json<Snapshot>| content.0)
            .fetch_optional(conn.as_mut())
            .await
            .map_err(QueryError::from)
    }

    async fn delete(&self, project_id: &str, version: &str) -> Result<(), QueryError> {
        const DELETE_QUERY: &'static str = r#"
            DELETE FROM project_snapshots
            WHERE project_id = $1 AND version = $2
        "#;

        let mut conn = self.connection_pool
            .get()
            .await?;

        sqlx::query(DELETE_QUERY)
            .bind(project_id)
            .bind(version)
            .execute(conn.as_mut())
            .await
            .map(|_| ())
            .map_err(QueryError::from)
    }
}