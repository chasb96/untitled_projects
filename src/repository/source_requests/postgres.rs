use futures::TryStreamExt;
use sqlx::{postgres::PgRow, types::Json};
use sqlx::Row;

use crate::repository::{error::QueryError, postgres::PostgresDatabase};

use super::{Approvable, Completable, CreateSourceRequest, SourceRequest, SourceRequestRepository, SourceRequestSummary};

impl SourceRequestRepository for PostgresDatabase {
    async fn get_by_id<'a>(&self, id: &'a str) -> Result<Option<SourceRequest>, QueryError> {
        const GET_BY_ID_QUERY: &'static str = r#"
            SELECT content
            FROM project_source_requests
            WHERE id = $1
        "#;

        let mut conn = self.connection_pool
            .get()
            .await?;

        let source_request = sqlx::query(GET_BY_ID_QUERY)
            .bind(id)
            .map(|row: PgRow| row.get("content"))
            .map(|content: Json<SourceRequest>| content.0)
            .fetch_optional(conn.as_mut())
            .await
            .map_err(QueryError::from)?;

        Ok(source_request)
    }

    async fn get_approvable<'a>(&self, id: &'a str) -> Result<Option<Approvable>, QueryError> {
        const GET_APPROVABLE_QUERY: &'static str = r#"
            SELECT content
            FROM project_source_requests
            WHERE id = $1 AND state IN (0, 1)
        "#;

        let mut conn = self.connection_pool
            .get()
            .await?;

        let source_request = sqlx::query(GET_APPROVABLE_QUERY)
            .bind(id)
            .map(|row: PgRow| row.get("content"))
            .map(|content: Json<Approvable>| content.0)
            .fetch_optional(conn.as_mut())
            .await
            .map_err(QueryError::from)?;

        Ok(source_request)
    }

    async fn get_completable<'a>(&self, id: &'a str) -> Result<Option<Completable>, QueryError> {
        const GET_COMPLETABLE_QUERY: &'static str = r#"
            SELECT content
            FROM project_source_requests
            WHERE id = $1 AND state IN (1, 2)
        "#;

        let mut conn = self.connection_pool
            .get()
            .await?;

        let source_request = sqlx::query(GET_COMPLETABLE_QUERY)
            .bind(id)
            .map(|row: PgRow| row.get("content"))
            .map(|content: Json<Completable>| content.0)
            .fetch_optional(conn.as_mut())
            .await
            .map_err(QueryError::from)?;

        Ok(source_request)
    }

    async fn create<'a>(&self, id: &'a str, source_request: impl Into<CreateSourceRequest<'a>>) -> Result<(), QueryError> {
        const CREATE_QUERY: &'static str = r#"
            INSERT INTO project_source_requests 
                (id, project_id, user_id, state, content)
            VALUES 
                ($1, $2, $3, $4. $5)
        "#;

        let mut conn = self.connection_pool
            .get()
            .await?;

        let source_request = source_request.into();

        sqlx::query(CREATE_QUERY)
            .bind(id)
            .bind(&source_request.project_id())
            .bind(&source_request.user_id())
            .bind(source_request.state_i16())
            .bind(Json::from(&source_request))
            .execute(conn.as_mut())
            .await
            .map(|_| ())
            .map_err(QueryError::from)
    }

    async fn update<'a>(&self, id: &'a str, source_request: impl Into<SourceRequest>) -> Result<(), QueryError> {
        const UPDATE_QUERY: &'static str = r#"
            UPDATE project_source_requests
            SET user_id = $2, state = $3, content = $4
            WHERE id = $1
        "#;

        let mut conn = self.connection_pool
            .get()
            .await?;

        let source_request = source_request.into();

        sqlx::query(UPDATE_QUERY)
            .bind(id)
            .bind(source_request.user_id())
            .bind(source_request.state_i16())
            .bind(Json::from(&source_request))
            .execute(conn.as_mut())
            .await
            .map_err(QueryError::from)?;

        Ok(())
    }

    async fn list_by_project_id(&self, project_id: &str) -> Result<Vec<(String, SourceRequestSummary)>, QueryError> {
        const LIST_BY_PROJECT_ID_QUERY: &'static str = r#"
            SELECT id, content
            FROM project_source_requests
            WHERE project_id = $1
            ORDER BY id DESC
        "#;

        let mut conn = self.connection_pool
            .get()
            .await?;

        let mut source_request_stream = sqlx::query(LIST_BY_PROJECT_ID_QUERY)
            .bind(project_id)
            .map(|row: PgRow| (row.get("id"), row.get("content")))
            .map(|content: (String, Json<SourceRequestSummary>)| (content.0, content.1.0))
            .fetch(conn.as_mut());

        let mut source_requests = Vec::new();

        while let Some(source_request) = source_request_stream.try_next().await? {
            source_requests.push(source_request);
        }

        Ok(source_requests)
    }

    async fn delete<'a>(&self, id: &'a str) -> Result<(), QueryError> {
        const DELETE_QUERY: &'static str = r#"
            DELETE FROM project_source_requests
            WHERE id = $1
        "#;

        let mut conn = self.connection_pool
            .get()
            .await?;

        sqlx::query(DELETE_QUERY)
            .bind(id)
            .execute(conn.as_mut())
            .await
            .map_err(QueryError::from)?;

        Ok(())
    }
}