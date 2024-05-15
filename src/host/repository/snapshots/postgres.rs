use sqlx::{postgres::PgRow, types::Json};
use sqlx::Row;

use crate::host::{events::Snapshot, repository::{error::QueryError, postgres::PostgresDatabase}};

use super::SnapshotsRepository;

impl SnapshotsRepository for PostgresDatabase {
    async fn create(&self, project_id: &str, version: &str, snapshot: impl Into<crate::host::events::Snapshot>) -> Result<(), crate::host::repository::error::QueryError> {
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
            .map_err(QueryError::from)?;

        Ok(())
    }

    async fn get_by_id(&self, project_id: &str, version: &str) -> Result<Option<crate::host::events::Snapshot>, crate::host::repository::error::QueryError> {
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
}