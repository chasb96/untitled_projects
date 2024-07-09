use sqlx::{postgres::PgRow, types::Json};
use sqlx::Row;

use crate::{events::Snapshot, repository::{error::QueryError, postgres::PostgresDatabase}};

use super::period::Period;
use super::ranking::Ranking;
use super::SnapshotsRepository;

impl SnapshotsRepository for PostgresDatabase {
    async fn list(&self, ranking: impl Into<Ranking>, period: impl Into<Period>, limit: i32) -> Result<Vec<Snapshot>, QueryError> {
        let ranking: Ranking =  ranking.into();
        let period: Period = period.into();
        
        let query = format!(r#"
            SELECT content
            FROM project_snapshots ps
                LEFT JOIN project_metrics pm
                    ON ps.project_id = pm.project_id
            WHERE {}
            ORDER BY {}
            LIMIT $1
        "#, period.as_where_clause(), ranking.as_ordering_clause());

        let mut conn = self.connection_pool
            .get()
            .await?;

        sqlx::query(&query)
            .bind(limit)
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