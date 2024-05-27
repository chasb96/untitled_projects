use crate::host::repository::{error::QueryError, postgres::PostgresDatabase};

use super::MetricsRepository;

impl MetricsRepository for PostgresDatabase {
    async fn increment_view_count(&self, project_id: &str) -> Result<(), QueryError> {
        const INCREMENT_QUERY: &'static str = r#"
            INSERT INTO project_metrics (project_id, view_count)
            VALUES ($1, 1)
            ON CONFLICT (project_id)
            DO UPDATE SET view_count = project_metrics.view_count + 1
        "#;

        let mut conn = self.connection_pool
            .get()
            .await?;

        sqlx::query(INCREMENT_QUERY)
            .bind(project_id)
            .execute(conn.as_mut())
            .await
            .map_err(QueryError::from)?;

        Ok(())
    }
}