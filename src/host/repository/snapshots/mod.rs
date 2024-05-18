mod postgres;

use crate::host::events::Snapshot;

use super::{error::QueryError, postgres::PostgresDatabase};

pub trait SnapshotsRepository {
    async fn create(&self, project_id: &str, version: &str, snapshot: impl Into<Snapshot>) -> Result<(), QueryError>;

    async fn get_by_id(&self, project_id: &str, version: &str) -> Result<Option<Snapshot>, QueryError>;

    async fn delete(&self, project_id: &str, version: &str) -> Result<(), QueryError>;
}

pub enum SnapshotsRepositoryOption {
    Postgres(PostgresDatabase),
}

impl SnapshotsRepository for SnapshotsRepositoryOption {
    async fn create(&self, project_id: &str, version: &str, snapshot: impl Into<Snapshot>) -> Result<(), QueryError> {
        match self {
            SnapshotsRepositoryOption::Postgres(pg) => pg.create(project_id, version, snapshot).await,
        }
    }

    async fn get_by_id(&self, project_id: &str, version: &str) -> Result<Option<Snapshot>, QueryError> {
        match self {
            SnapshotsRepositoryOption::Postgres(pg) => pg.get_by_id(project_id, version).await,
        }
    }

    async fn delete(&self, project_id: &str, version: &str) -> Result<(), QueryError> {
        match self {
            SnapshotsRepositoryOption::Postgres(pg) => pg.delete(project_id, version).await,
        }
    }
}

impl Default for SnapshotsRepositoryOption {
    fn default() -> Self {
        Self::Postgres(PostgresDatabase::default())
    }
}