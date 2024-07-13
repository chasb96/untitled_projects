mod postgres;

use crate::repository::{error::QueryError, postgres::PostgresDatabase};

pub struct SourceRequestComment {
    pub id: i32,
    pub source_request_id: i32,
    pub user_id: String,
    pub content: String,
    pub created_at: chrono::NaiveDateTime,
}

pub struct CreateSourceRequestComment<'a> {
    pub source_request_id: &'a str,
    pub user_id: &'a str,
    pub content: &'a str,
    pub created_at: &'a chrono::NaiveDateTime,
}

pub trait SourceRequestCommentRepository {
    async fn create<'a>(&self, source_request_comment: CreateSourceRequestComment<'a>) -> Result<i32, QueryError>;

    async fn list<'a>(&self, source_request_id: &'a str) -> Result<Vec<SourceRequestComment>, QueryError>;
}

pub enum SourceRequestCommentRepositoryOption {
    Postgres(PostgresDatabase),
}

impl SourceRequestCommentRepository for SourceRequestCommentRepositoryOption {
    async fn create<'a>(&self, source_request_comment: CreateSourceRequestComment<'a>) -> Result<i32, QueryError> {
        match self {
            SourceRequestCommentRepositoryOption::Postgres(pg) => pg.create(source_request_comment).await,
        }
    }

    async fn list<'a>(&self, source_request_id: &'a str) -> Result<Vec<SourceRequestComment>, QueryError> {
        match self {
            SourceRequestCommentRepositoryOption::Postgres(pg) => pg.list(source_request_id).await,
        }
    }
}

impl Default for SourceRequestCommentRepositoryOption {
    fn default() -> Self {
        Self::Postgres(Default::default())
    }
}