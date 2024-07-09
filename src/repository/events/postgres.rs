use futures::TryStreamExt;
use sqlx::{postgres::PgRow, types::Json};
use sqlx::Row;

use crate::{events::{ Event, EventKind }, repository::{error::QueryError, postgres::PostgresDatabase}};

use super::EventsRepository;

impl EventsRepository for PostgresDatabase {
    async fn create(&self, project_id: &str, event: impl Into<EventKind>) -> Result<(), QueryError> {
        const CREATE_QUERY: &'static str = r#"
            INSERT INTO project_events (project_id, event_id, content)
            VALUES ($1, $2, $3)
        "#;

        let mut conn = self.connection_pool
            .get()
            .await?;

        let event = event.into();

        sqlx::query(CREATE_QUERY)
            .bind(project_id)
            .bind(&event.event_id())
            .bind(Json::from(&event))
            .execute(conn.as_mut())
            .await
            .map_err(QueryError::from)?;

        Ok(())
    }

    async fn list(&self, project_id: &str) -> Result<Vec<EventKind>, QueryError> {
        const LIST_QUERY: &'static str = r#"
            SELECT content
            FROM project_events
            WHERE project_id = $1
            ORDER BY id ASC
        "#;

        let mut conn = self.connection_pool
            .get()
            .await.unwrap();

        let mut event_stream = sqlx::query(LIST_QUERY)
            .bind(project_id)
            .map(|row: PgRow| row.get("content"))
            .map(|content: Json<EventKind>| content.0)
            .fetch(conn.as_mut());

        let mut events = Vec::new();

        while let Some(event) = event_stream.try_next().await? {
            events.push(event);
        }

        Ok(events)
    }

    async fn list_until(&self, project_id: &str, event_id: &str) -> Result<Vec<EventKind>, QueryError> {
        const LIST_QUERY: &'static str = r#"
            SELECT content
            FROM project_events
            WHERE project_id = $1 AND id <= (
                SELECT id
                FROM project_events
                WHERE project_id = $1 AND event_id = $2
            )
            ORDER BY id ASC
        "#;

        let mut conn = self.connection_pool
            .get()
            .await.unwrap();

        let mut event_stream = sqlx::query(LIST_QUERY)
            .bind(project_id)
            .bind(event_id)
            .map(|row: PgRow| row.get("content"))
            .map(|content: Json<EventKind>| content.0)
            .fetch(conn.as_mut());

        let mut events = Vec::new();

        while let Some(event) = event_stream.try_next().await? {
            events.push(event);
        }

        Ok(events)
    }
}