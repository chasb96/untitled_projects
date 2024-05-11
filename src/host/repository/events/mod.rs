mod error;
mod postgres;

use crate::host::events::{EventKind, Snapshot};

use self::error::QueryError;

use super::postgres::PostgresDatabase;

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