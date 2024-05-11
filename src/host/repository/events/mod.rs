mod error;
mod postgres;

use chrono::{DateTime, Utc};
use sqlx::postgres::PgRow;
use sqlx::types::Json;
use sqlx::Row;

use crate::host::events::{EventKind, Snapshot};

use self::error::QueryError;

use super::postgres::PostgresDatabase;

struct Event {
    pub id: i32,
    pub project_id: String,
    pub content: EventKind,
}

impl From<PgRow> for Event {
    fn from(value: PgRow) -> Self {
        let content: Json<EventKind> = value.get("content");

        Event {
            id: value.get("id"),
            project_id: value.get("project_id"),
            content: content.0,
        }
    }
}

pub trait EventsRepository {
    async fn create(&self, project_id: &str, event: EventKind) -> Result<(), QueryError>;

    async fn get_by_id(&self, project_id: &str) -> Result<Option<Snapshot>, QueryError>;
}

pub enum EventsRepositoryOption {
    Postgres(PostgresDatabase),
}

impl EventsRepository for EventsRepositoryOption {
    async fn create(&self, project_id: &str, event: EventKind) -> Result<(), QueryError> {
        match self {
            EventsRepositoryOption::Postgres(pg) => pg.create(project_id, event).await,
        }
    }

    async fn get_by_id(&self, project_id: &str) -> Result<Option<Snapshot>, QueryError> {
        match self {
            EventsRepositoryOption::Postgres(pg) => pg.get_by_id(project_id).await,
        }
    }
}

impl Default for EventsRepositoryOption {
    fn default() -> Self {
        Self::Postgres(PostgresDatabase::default())
    }
}