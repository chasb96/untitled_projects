use futures::TryStreamExt;
use sqlx::types::Json;
use sqlx::Row;

use crate::host::{events::{EventKind, Snapshot}, repository::postgres::PostgresDatabase};

use super::{error::QueryError, EventsRepository};

impl EventsRepository for PostgresDatabase {
    async fn create(&self, project_id: &str, event: EventKind) -> Result<(), QueryError> {
        const CREATE_QUERY: &'static str = r#"
            INSERT INTO project_events (project_id, content)
            VALUES ($1, $2)
        "#;

        let mut conn = self.connection_pool
            .get()
            .await?;

        sqlx::query(CREATE_QUERY)
            .bind(project_id)
            .bind(Json::from(event))
            .execute(conn.as_mut())
            .await
            .map_err(QueryError::from)?;

        Ok(())
    }

    async fn get_by_id(&self, project_id: &str) -> Result<Option<Snapshot>, QueryError> {
        const LIST_QUERY: &'static str = r#"
            SELECT
                content
            FROM
                project_events
            WHERE
                project_id = $1
            ORDER BY
                id ASC
        "#;

        let mut conn = self.connection_pool
            .get()
            .await?;

        let mut records = sqlx::query(LIST_QUERY)
            .bind(project_id)
            .fetch(conn.as_mut());

        let mut records_found = false;
        let mut snapshot = Snapshot::new();

        while let Some(item) = records.try_next().await? {
            records_found = true;

            let content: Json<EventKind> = item.get("content");

            snapshot.apply_event(content.0);
        }

        Ok(if records_found { Some(snapshot) } else { None })
    }
}