mod cache;
mod mongo;

use cache::SnapshotsCachingRepository;

use crate::events::Snapshot;

use super::mongo::MongoDatabase;
use super::error::QueryError;

pub enum ListQuery {
    ProjectIds { project_ids: Vec<String> },
    UserId { user_id: String },
}

pub trait SnapshotsRepository {
    async fn list(&self, query: &ListQuery) -> Result<Vec<Snapshot>, QueryError>;

    async fn create(&self, project_id: &str, version: &str, snapshot: impl Into<Snapshot>) -> Result<(), QueryError>;

    async fn get_by_id(&self, project_id: &str, version: &str) -> Result<Option<Snapshot>, QueryError>;

    async fn delete(&self, project_id: &str, version: &str) -> Result<(), QueryError>;
}

#[allow(dead_code)]
pub enum SnapshotsRepositoryOption {
    Mongo(MongoDatabase),
    CachedMongo(SnapshotsCachingRepository<MongoDatabase>),
}

impl SnapshotsRepository for SnapshotsRepositoryOption {
    async fn list(&self, project_ids: &ListQuery) -> Result<Vec<Snapshot>, QueryError> {
        match self {
            Self::Mongo(mongo) => mongo.list(project_ids).await,
            Self::CachedMongo(cached_mongo) => cached_mongo.list(project_ids).await,
        }
    }

    async fn create(&self, project_id: &str, version: &str, snapshot: impl Into<Snapshot>) -> Result<(), QueryError> {
        match self {
            Self::Mongo(mongo) => mongo.create(project_id, version, snapshot).await,
            Self::CachedMongo(cached_mongo) => cached_mongo.create(project_id, version, snapshot).await,
        }
    }

    async fn get_by_id(&self, project_id: &str, version: &str) -> Result<Option<Snapshot>, QueryError> {
        match self {
            Self::Mongo(mongo) => mongo.get_by_id(project_id, version).await,
            Self::CachedMongo(cached_mongo) => cached_mongo.get_by_id(project_id, version).await,
        }
    }

    async fn delete(&self, project_id: &str, version: &str) -> Result<(), QueryError> {
        match self {
            Self::Mongo(mongo) => mongo.delete(project_id, version).await,
            Self::CachedMongo(cached_mongo) => cached_mongo.delete(project_id, version).await,
        }
    }
}

impl Default for SnapshotsRepositoryOption {
    fn default() -> Self {
        Self::CachedMongo(Default::default())
    }
}