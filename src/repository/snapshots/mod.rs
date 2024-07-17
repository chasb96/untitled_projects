mod postgres;
mod cache;
mod mongo;

use cache::SnapshotsCachingRepository;

use crate::events::Snapshot;

use super::{error::QueryError, mongo::MongoDatabase, postgres::PostgresDatabase};

pub trait SnapshotsRepository {
    async fn list(&self, project_ids: &Option<Vec<String>>) -> Result<Vec<Snapshot>, QueryError>;

    async fn create(&self, project_id: &str, version: &str, snapshot: impl Into<Snapshot>) -> Result<(), QueryError>;

    async fn get_by_id(&self, project_id: &str, version: &str) -> Result<Option<Snapshot>, QueryError>;

    async fn delete(&self, project_id: &str, version: &str) -> Result<(), QueryError>;
}

#[allow(dead_code)]
pub enum SnapshotsRepositoryOption {
    Postgres(PostgresDatabase),
    CachedPostgres(SnapshotsCachingRepository<PostgresDatabase>),
    Mongo(MongoDatabase),
    CachedMongo(SnapshotsCachingRepository<MongoDatabase>),
}

impl SnapshotsRepository for SnapshotsRepositoryOption {
    async fn list(&self, project_ids: &Option<Vec<String>>) -> Result<Vec<Snapshot>, QueryError> {
        match self {
            Self::Postgres(pg) => pg.list(project_ids).await,
            Self::CachedPostgres(cached_pg) => cached_pg.list(project_ids).await,
            Self::Mongo(mongo) => mongo.list(project_ids).await,
            Self::CachedMongo(cached_mongo) => cached_mongo.list(project_ids).await,
        }
    }

    async fn create(&self, project_id: &str, version: &str, snapshot: impl Into<Snapshot>) -> Result<(), QueryError> {
        match self {
            Self::Postgres(pg) => pg.create(project_id, version, snapshot).await,
            Self::CachedPostgres(cached_pg) => cached_pg.create(project_id, version, snapshot).await,
            Self::Mongo(mongo) => mongo.create(project_id, version, snapshot).await,
            Self::CachedMongo(cached_mongo) => cached_mongo.create(project_id, version, snapshot).await,
        }
    }

    async fn get_by_id(&self, project_id: &str, version: &str) -> Result<Option<Snapshot>, QueryError> {
        match self {
            Self::Postgres(pg) => pg.get_by_id(project_id, version).await,
            Self::CachedPostgres(cached_pg) => cached_pg.get_by_id(project_id, version).await,
            Self::Mongo(mongo) => mongo.get_by_id(project_id, version).await,
            Self::CachedMongo(cached_mongo) => cached_mongo.get_by_id(project_id, version).await,
        }
    }

    async fn delete(&self, project_id: &str, version: &str) -> Result<(), QueryError> {
        match self {
            Self::Postgres(pg) => pg.delete(project_id, version).await,
            Self::CachedPostgres(cached_pg) => cached_pg.delete(project_id, version).await,
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