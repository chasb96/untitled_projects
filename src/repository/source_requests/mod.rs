mod postgres;
mod mongo;
mod new;
mod approved;
mod approvable;
mod source_request;
mod completed;
mod completable;
pub mod comments;

pub use new::{NewSourceRequest, NewSourceRequestSummary, CreateNewSourceRequest};
pub use approved::{ApprovedSourceRequest, ApprovedSourceRequestSummary};
pub use approvable::Approvable;
pub use source_request::CreateSourceRequest;
pub use source_request::{SourceRequest, SourceRequestSummary, FileMap, NewFileMap};
pub use completed::{CompletedSourceRequest, CompletedSourceRequestSummary};
pub use completable::Completable;

use super::{error::QueryError, postgres::PostgresDatabase};

pub trait SourceRequestRepository {
    async fn get_by_id<'a>(&self, id: &'a str) -> Result<Option<SourceRequest>, QueryError>;

    async fn get_approvable<'a>(&self, id: &'a str) -> Result<Option<Approvable>, QueryError>;

    async fn get_completable<'a>(&self, id: &'a str) -> Result<Option<Completable>, QueryError>;

    async fn create<'a>(&self, id: &'a str, source_request: impl Into<CreateSourceRequest<'a>>) -> Result<(), QueryError>;

    async fn update<'a>(&self, id: &'a str, source_request: impl Into<SourceRequest>) -> Result<(), QueryError>;

    async fn list_by_project_id(&self, project_id: &str) -> Result<Vec<(String, SourceRequestSummary)>, QueryError>;

    async fn delete<'a>(&self, id: &'a str) -> Result<(), QueryError>;
}

pub enum SourceRequestRepositoryOption {
    Postgres(PostgresDatabase),
}

impl SourceRequestRepository for SourceRequestRepositoryOption {
    async fn get_by_id<'a>(&self, id: &'a str) -> Result<Option<SourceRequest>, QueryError> {
        match self {
            SourceRequestRepositoryOption::Postgres(pg) => pg.get_by_id(id).await,
        }
    }

    async fn get_approvable<'a>(&self, id: &'a str) -> Result<Option<Approvable>, QueryError> {
        match self {
            SourceRequestRepositoryOption::Postgres(pg) => pg.get_approvable(id).await,
        }
    }

    async fn get_completable<'a>(&self, id: &'a str) -> Result<Option<Completable>, QueryError> {
        match self {
            SourceRequestRepositoryOption::Postgres(pg) => pg.get_completable(id).await,
        }
    }

    async fn create<'a>(&self, id: &'a str, source_request: impl Into<CreateSourceRequest<'a>>) -> Result<(), QueryError> {
        match self {
            SourceRequestRepositoryOption::Postgres(pg) => pg.create(id, source_request).await,
        }
    }

    async fn update<'a>(&self, id: &'a str, source_request: impl Into<SourceRequest>) -> Result<(), QueryError> {
        match self {
            SourceRequestRepositoryOption::Postgres(pg) => pg.update(id, source_request).await,
        }
    }

    async fn list_by_project_id(&self, project_id: &str) -> Result<Vec<(String, SourceRequestSummary)>, QueryError> {
        match self {
            SourceRequestRepositoryOption::Postgres(pg) => pg.list_by_project_id(project_id).await,
        }
    }

    async fn delete<'a>(&self, id: &'a str) -> Result<(), QueryError> {
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