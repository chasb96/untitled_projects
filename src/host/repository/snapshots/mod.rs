mod postgres;
mod ranking;
mod period;
mod cache;

use cache::SnapshotsCachingRepository;
pub use ranking::Ranking;
pub use period::Period;

use crate::host::events::Snapshot;

use super::{error::QueryError, postgres::PostgresDatabase};

pub trait SnapshotsRepository {
    async fn list(&self, ranking: impl Into<Ranking>, period: impl Into<Period>, limit: i32) -> Result<Vec<Snapshot>, QueryError>;

    async fn create(&self, project_id: &str, version: &str, snapshot: impl Into<Snapshot>) -> Result<(), QueryError>;

    async fn get_by_id(&self, project_id: &str, version: &str) -> Result<Option<Snapshot>, QueryError>;

    async fn delete(&self, project_id: &str, version: &str) -> Result<(), QueryError>;
}

#[allow(dead_code)]
pub enum SnapshotsRepositoryOption {
    Postgres(PostgresDatabase),
    CachedPostgres(SnapshotsCachingRepository<PostgresDatabase>),
}

impl SnapshotsRepository for SnapshotsRepositoryOption {
    async fn list(&self, ranking: impl Into<Ranking>, period: impl Into<Period>, limit: i32) -> Result<Vec<Snapshot>, QueryError> {
        match self {
            SnapshotsRepositoryOption::Postgres(pg) => pg.list(ranking, period, limit).await,
            SnapshotsRepositoryOption::CachedPostgres(cached_pg) => cached_pg.list(ranking, period, limit).await,
        }
    }

    async fn create(&self, project_id: &str, version: &str, snapshot: impl Into<Snapshot>) -> Result<(), QueryError> {
        match self {
            SnapshotsRepositoryOption::Postgres(pg) => pg.create(project_id, version, snapshot).await,
            SnapshotsRepositoryOption::CachedPostgres(cached_pg) => cached_pg.create(project_id, version, snapshot).await,
        }
    }

    async fn get_by_id(&self, project_id: &str, version: &str) -> Result<Option<Snapshot>, QueryError> {
        match self {
            SnapshotsRepositoryOption::Postgres(pg) => pg.get_by_id(project_id, version).await,
            SnapshotsRepositoryOption::CachedPostgres(cached_pg) => cached_pg.get_by_id(project_id, version).await,
        }
    }

    async fn delete(&self, project_id: &str, version: &str) -> Result<(), QueryError> {
        match self {
            SnapshotsRepositoryOption::Postgres(pg) => pg.delete(project_id, version).await,
            SnapshotsRepositoryOption::CachedPostgres(cached_pg) => cached_pg.delete(project_id, version).await,
        }
    }
}

impl Default for SnapshotsRepositoryOption {
    fn default() -> Self {
        Self::CachedPostgres(SnapshotsCachingRepository::default())
    }
}