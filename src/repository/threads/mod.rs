mod mongo;

use serde::Deserialize;

use super::mongo::MongoDatabase;
use super::error::QueryError;

pub struct NewThread<'a> {
    pub id: &'a str,
    pub project_id: &'a str,
    pub user_id: &'a str,
    pub title: &'a str,
}

#[derive(Deserialize)]
pub struct Thread {
    pub id: String,
    pub project_id: String,
    pub user_id: String,
    pub title: String,
}

pub struct NewComment<'a> {
    pub id: &'a str,
    pub thread_id: &'a str,
    pub user_id: &'a str,
    pub content: &'a str,
}

#[derive(Deserialize)]
pub struct Comment {
    pub id: String,
    pub thread_id: String,
    pub user_id: String,
    pub content: String,
}

pub trait ThreadsRepository {
    async fn create(&self, thread: NewThread) -> Result<(), QueryError>;

    async fn list(&self, project_id: &str) -> Result<Vec<Thread>, QueryError>;

    async fn get_by_id(&self, id: &str) -> Result<Option<Thread>, QueryError>;

    async fn create_comment(&self, comment: NewComment) -> Result<(), QueryError>;

    async fn list_comments(&self, thread_id: &str) -> Result<Vec<Comment>, QueryError>;
}

#[allow(dead_code)]
pub enum ThreadsRepositoryOption {
    Mongo(MongoDatabase),
}

impl ThreadsRepository for ThreadsRepositoryOption {
    async fn create<'a>(&self, thread: NewThread<'a>) -> Result<(), QueryError> {
        match self {
            Self::Mongo(mongo) => mongo.create(thread).await,
        }
    }

    async fn list(&self, project_id: &str) -> Result<Vec<Thread>, QueryError> {
        match self {
            Self::Mongo(mongo) => mongo.list(project_id).await,
        }
    }

    async fn get_by_id(&self, id: &str) -> Result<Option<Thread>, QueryError> {
        match self {
            Self::Mongo(mongo) => mongo.get_by_id(id).await,
        }
    }

    async fn create_comment<'a>(&self, comment: NewComment<'a>) -> Result<(), QueryError> {
        match self {
            Self::Mongo(mongo) => mongo.create_comment(comment).await,
        }
    }

    async fn list_comments(&self, thread_id: &str) -> Result<Vec<Comment>, QueryError> {
        match self {
            Self::Mongo(mongo) => mongo.list_comments(thread_id).await,
        }
    }
}

impl Default for ThreadsRepositoryOption {
    fn default() -> Self {
        Self::Mongo(Default::default())
    }
}
