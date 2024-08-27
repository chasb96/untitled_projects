mod mongo;

use crate::repository::mongo::MongoDatabase;
use crate::repository::error::QueryError;

pub struct SourceRequestComment {
    pub id: String,
    pub source_request_id: String,
    pub user_id: String,
    pub content: String,
}

pub struct CreateSourceRequestComment<'a> {
    pub id: &'a str,
    pub source_request_id: &'a str,
    pub user_id: &'a str,
    pub content: &'a str,
}

pub trait SourceRequestCommentRepository {
    async fn create<'a>(&self, source_request_comment: CreateSourceRequestComment<'a>) -> Result<(), QueryError>;

    async fn list<'a>(&self, source_request_id: &'a str) -> Result<Vec<SourceRequestComment>, QueryError>;
}

#[allow(dead_code)]
pub enum SourceRequestCommentRepositoryOption {
    Mongo(MongoDatabase),
}

impl SourceRequestCommentRepository for SourceRequestCommentRepositoryOption {
    async fn create<'a>(&self, source_request_comment: CreateSourceRequestComment<'a>) -> Result<(), QueryError> {
        match self {
            Self::Mongo(mongo) => mongo.create(source_request_comment).await,
        }
    }

    async fn list<'a>(&self, source_request_id: &'a str) -> Result<Vec<SourceRequestComment>, QueryError> {
        match self {
            Self::Mongo(mongo) => mongo.list(source_request_id).await,
        }
    }
}

impl Default for SourceRequestCommentRepositoryOption {
    fn default() -> Self {
        Self::Mongo(Default::default())
    }
}