mod postgres;

use crate::host::events::EventKind;

use super::{error::QueryError, postgres::PostgresDatabase};

pub trait EventsRepository {
    async fn create(&self, project_id: &str, event: impl Into<EventKind>) -> Result<(), QueryError>;

    async fn list(&self, project_id: &str) -> Result<Vec<EventKind>, QueryError>;

    async fn list_until(&self, project_id: &str, event_id: &str) -> Result<Vec<EventKind>, QueryError>;
}

pub enum EventsRepositoryOption {
    Postgres(PostgresDatabase),
}

impl EventsRepository for EventsRepositoryOption {
    async fn create(&self, project_id: &str, event: impl Into<EventKind>) -> Result<(), QueryError> {
        match self {
            EventsRepositoryOption::Postgres(pg) => pg.create(project_id, event).await,
        }
    }
    
    async fn list(&self, project_id: &str) -> Result<Vec<EventKind>, QueryError> {
        match self {
            EventsRepositoryOption::Postgres(pg) => pg.list(project_id).await,
        }
    }

    async fn list_until(&self, project_id: &str, event_id: &str) -> Result<Vec<EventKind>, QueryError> {
        match self {
            EventsRepositoryOption::Postgres(pg) => pg.list_until(project_id, event_id).await,
        }
    }
}

impl Default for EventsRepositoryOption {
    fn default() -> Self {
        Self::Postgres(PostgresDatabase::default())
    }
}