mod postgres;

use super::{error::QueryError, postgres::PostgresDatabase};

pub struct NewThread<'a> {
    pub project_id: &'a str,
    pub user_id: &'a str,
    pub title: &'a str,
    pub created_at: &'a chrono::NaiveDateTime,
}

pub struct Thread {
    pub id: i32,
    pub project_id: String,
    pub user_id: String,
    pub title: String,
    pub created_at: chrono::NaiveDateTime,
}

pub struct NewComment<'a> {
    pub thread_id: i32,
    pub user_id: &'a str,
    pub content: &'a str,
    pub created_at: &'a chrono::NaiveDateTime,
}

pub struct Comment {
    pub id: i32,
    pub thread_id: i32,
    pub user_id: String,
    pub content: String,
    pub created_at: chrono::NaiveDateTime,
}

pub trait ThreadsRepository {
    async fn create(&self, threat: NewThread) -> Result<i32, QueryError>;

    async fn list(&self, project_id: &str) -> Result<Vec<Thread>, QueryError>;

    async fn get_by_id(&self, id: i32) -> Result<Option<Thread>, QueryError>;

    async fn create_comment(&self, comment: NewComment) -> Result<i32, QueryError>;

    async fn list_comments(&self, thread_id: i32) -> Result<Vec<Comment>, QueryError>;
}

pub enum ThreadsRepositoryOption {
    Postgres(PostgresDatabase),
}

impl ThreadsRepository for ThreadsRepositoryOption {
    async fn create<'a>(&self, thread: NewThread<'a>) -> Result<i32, QueryError> {
        match self {
            ThreadsRepositoryOption::Postgres(pg) => pg.create(thread).await,
        }
    }

    async fn list(&self, project_id: &str) -> Result<Vec<Thread>, QueryError> {
        match self {
            ThreadsRepositoryOption::Postgres(pg) => pg.list(project_id).await,
        }
    }

    async fn get_by_id(&self, id: i32) -> Result<Option<Thread>, QueryError> {
        match self {
            ThreadsRepositoryOption::Postgres(pg) => pg.get_by_id(id).await,
        }
    }

    async fn create_comment<'a>(&self, comment: NewComment<'a>) -> Result<i32, QueryError> {
        match self {
            ThreadsRepositoryOption::Postgres(pg) => pg.create_comment(comment).await,
        }
    }

    async fn list_comments(&self, thread_id: i32) -> Result<Vec<Comment>, QueryError> {
        match self {
            ThreadsRepositoryOption::Postgres(pg) => pg.list_comments(thread_id).await,
        }
    }
}

impl Default for ThreadsRepositoryOption {
    fn default() -> Self {
        Self::Postgres(PostgresDatabase::default())
    }
}
