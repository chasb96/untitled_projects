mod postgres;

use super::{error::QueryError, postgres::PostgresDatabase};

pub trait TagsRepository {
    async fn list(&self, project_id: &str) -> Result<Vec<String>, QueryError>;

    async fn create(&self, project_id: &str, tag: &str) -> Result<(), QueryError>;

    async fn delete(&self, project_id: &str, tag: &str) -> Result<(), QueryError>;
}

pub enum TagsRepositoryOption {
    Postgres(PostgresDatabase),
}

impl TagsRepository for TagsRepositoryOption {
    async fn list(&self, project_id: &str) -> Result<Vec<String>, QueryError> {
        match self {
            TagsRepositoryOption::Postgres(pg) => pg.list(project_id).await,
        }
    }

    async fn create(&self, project_id: &str, tag: &str) -> Result<(), QueryError> {
        match self {
            TagsRepositoryOption::Postgres(pg) => pg.create(project_id, tag).await,
        }
    }

    async fn delete(&self, project_id: &str, tag: &str) -> Result<(), QueryError> {
        match self {
            TagsRepositoryOption::Postgres(pg) => pg.delete(project_id, tag).await,
        }
    }
}

impl Default for TagsRepositoryOption {
    fn default() -> Self {
        TagsRepositoryOption::Postgres(Default::default())
    }
}