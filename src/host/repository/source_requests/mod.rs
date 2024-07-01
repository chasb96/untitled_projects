mod postgres;
mod new;
mod approved;
mod approvable;
mod source_request;
mod completed;
mod completable;

pub use new::{NewSourceRequest, NewSourceRequestSummary, CreateNewSourceRequest};
pub use approved::{ApprovedSourceRequest, ApprovedSourceRequestSummary};
pub use approvable::Approvable;
pub use source_request::CreateSourceRequest;
pub use source_request::{SourceRequest, SourceRequestSummary, FileMap, NewFileMap};
pub use completed::{CompletedSourceRequest, CompletedSourceRequestSummary};
pub use completable::Completable;

use super::{error::QueryError, postgres::PostgresDatabase};

pub trait SourceRequestRepository {
    async fn get_by_id(&self, id: i32) -> Result<Option<SourceRequest>, QueryError>;

    async fn get_approvable(&self, id: i32) -> Result<Option<Approvable>, QueryError>;

    async fn get_completable(&self, id: i32) -> Result<Option<Completable>, QueryError>;

    async fn create<'a>(&self, source_request: impl Into<CreateSourceRequest<'a>>) -> Result<i32, QueryError>;

    async fn update(&self, id: i32, source_request: impl Into<SourceRequest>) -> Result<(), QueryError>;

    async fn list_by_project_id(&self, project_id: &str) -> Result<Vec<(i32, SourceRequestSummary)>, QueryError>;

    async fn delete(&self, id: i32) -> Result<(), QueryError>;
}

pub enum SourceRequestRepositoryOption {
    Postgres(PostgresDatabase),
}

impl SourceRequestRepository for SourceRequestRepositoryOption {
    async fn get_by_id(&self, id: i32) -> Result<Option<SourceRequest>, QueryError> {
        match self {
            SourceRequestRepositoryOption::Postgres(pg) => pg.get_by_id(id).await,
        }
    }

    async fn get_approvable(&self, id: i32) -> Result<Option<Approvable>, QueryError> {
        match self {
            SourceRequestRepositoryOption::Postgres(pg) => pg.get_approvable(id).await,
        }
    }

    async fn get_completable(&self, id: i32) -> Result<Option<Completable>, QueryError> {
        match self {
            SourceRequestRepositoryOption::Postgres(pg) => pg.get_completable(id).await,
        }
    }

    async fn create<'a>(&self, source_request: impl Into<CreateSourceRequest<'a>>) -> Result<i32, QueryError> {
        match self {
            SourceRequestRepositoryOption::Postgres(pg) => pg.create(source_request).await,
        }
    }

    async fn update(&self, id: i32, source_request: impl Into<SourceRequest>) -> Result<(), QueryError> {
        match self {
            SourceRequestRepositoryOption::Postgres(pg) => pg.update(id, source_request).await,
        }
    }

    async fn list_by_project_id(&self, project_id: &str) -> Result<Vec<(i32, SourceRequestSummary)>, QueryError> {
        match self {
            SourceRequestRepositoryOption::Postgres(pg) => pg.list_by_project_id(project_id).await,
        }
    }

    async fn delete(&self, id: i32) -> Result<(), QueryError> {
        match self {
            SourceRequestRepositoryOption::Postgres(pg) => pg.delete(id).await,
        }
    }
}

impl Default for SourceRequestRepositoryOption {
    fn default() -> Self {
        SourceRequestRepositoryOption::Postgres(PostgresDatabase::default())
    }
}