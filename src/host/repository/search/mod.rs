mod postgres;

use super::{error::QueryError, postgres::PostgresDatabase};

pub struct SearchRecord {
    pub project_id: String,
    pub name: String,
    pub score: f32,
}

pub trait SearchRepository {
    async fn create(&self, project_id: &str, name: &str, value: &str) -> Result<(), QueryError>;

    async fn query(&self, terms: Vec<&str>) -> Result<Vec<SearchRecord>, QueryError>;
}

pub enum SearchRepositoryOption {
    Postgres(PostgresDatabase),
}

impl SearchRepository for SearchRepositoryOption {
    async fn create(&self, project_id: &str, name: &str, value: &str) -> Result<(), QueryError> {
        match self {
            SearchRepositoryOption::Postgres(db) => db.create(project_id, name, value).await,
        }
    }

    async fn query(&self, terms: Vec<&str>) -> Result<Vec<SearchRecord>, QueryError> {
        match self {
            SearchRepositoryOption::Postgres(db) => db.query(terms).await,
        }
    }
}

impl Default for SearchRepositoryOption {
    fn default() -> Self {
        SearchRepositoryOption::Postgres(Default::default())
    }
}