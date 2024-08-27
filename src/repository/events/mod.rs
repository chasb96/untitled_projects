mod mongo;

use crate::events::EventKind;

use super::mongo::MongoDatabase;
use super::error::QueryError;

pub trait EventsRepository {
    async fn create(&self, project_id: &str, event: impl Into<EventKind>) -> Result<(), QueryError>;

    async fn list(&self, project_id: &str, event_id: &str) -> Result<Vec<EventKind>, QueryError>;
}

#[allow(dead_code)]
pub enum EventsRepositoryOption {
    Mongo(MongoDatabase),
}

impl EventsRepository for EventsRepositoryOption {
    async fn create(&self, project_id: &str, event: impl Into<EventKind>) -> Result<(), QueryError> {
        match self {
            Self::Mongo(mongo) => mongo.create(project_id, event).await,
        }
    }

    async fn list(&self, project_id: &str, event_id: &str) -> Result<Vec<EventKind>, QueryError> {
        match self {
            Self::Mongo(mongo) => mongo.list(project_id, event_id).await,
        }
    }
}

impl Default for EventsRepositoryOption {
    fn default() -> Self {
        Self::Mongo(Default::default())
    }
}