use super::{error::QueryError, postgres::PostgresDatabase};

mod postgres;

pub trait MetricsRepository {
    async fn increment_view_count(&self, project_id: &str) -> Result<(), QueryError>;
}

pub enum MetricsRepositoryOption {
    Postgres(PostgresDatabase),
}

impl MetricsRepository for MetricsRepositoryOption {
    async fn increment_view_count(&self, project_id: &str) -> Result<(), QueryError> {
        match self {
            MetricsRepositoryOption::Postgres(pg) => pg.increment_view_count(project_id).await,
        }
    }
}

impl Default for MetricsRepositoryOption {
    fn default() -> Self {
        Self::Postgres(PostgresDatabase::default())
    }
}