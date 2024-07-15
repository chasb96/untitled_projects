mod postgres;
mod mongo;

use super::{error::QueryError, mongo::MongoDatabase, postgres::PostgresDatabase};

pub trait TagsRepository {
    async fn list(&self, project_id: &str) -> Result<Vec<String>, QueryError>;

    async fn create(&self, project_id: &str, tag: &str) -> Result<(), QueryError>;

    async fn delete(&self, project_id: &str, tag: &str) -> Result<(), QueryError>;
}

#[allow(dead_code)]
pub enum TagsRepositoryOption {
    Postgres(PostgresDatabase),
    Mongo(MongoDatabase),
}

impl TagsRepository for TagsRepositoryOption {
    async fn list(&self, project_id: &str) -> Result<Vec<String>, QueryError> {
        match self {
            Self::Postgres(pg) => pg.list(project_id).await,
            Self::Mongo(mongo) => mongo.list(project_id).await,
        }
    }

    async fn create(&self, project_id: &str, tag: &str) -> Result<(), QueryError> {
        match self {
            Self::Postgres(pg) => pg.create(project_id, tag).await,
            Self::Mongo(mongo) => mongo.create(project_id, tag).await,
        }
    }

    async fn delete(&self, project_id: &str, tag: &str) -> Result<(), QueryError> {
        match self {
            Self::Postgres(pg) => pg.delete(project_id, tag).await,
            Self::Mongo(mongo) => mongo.delete(project_id, tag).await,
        }
    }
}

impl Default for TagsRepositoryOption {
    fn default() -> Self {
        Self::Mongo(Default::default())
    }
}