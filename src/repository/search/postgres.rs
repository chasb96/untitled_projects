use std::collections::HashMap;
use futures::TryStreamExt;
use sqlx::postgres::PgRow;
use sqlx::Row;

use crate::repository::{error::QueryError, postgres::PostgresDatabase};

use super::{SearchRecord, SearchRepository};

impl SearchRepository for PostgresDatabase {
    async fn create(&self, project_id: &str, name: &str, value: &str) -> Result<(), QueryError> {
        const INSERT_QUERY: &'static str = r#"
            INSERT INTO projects_search (project_id, name, value, code)
            VALUES ($1, $2, $3, DMETAPHONE($3))
        "#;

        let mut conn = self.connection_pool
            .get()
            .await?;

        sqlx::query(INSERT_QUERY)
            .bind(project_id)
            .bind(name)
            .bind(value)
            .execute(conn.as_mut())
            .await
            .map(|_| ())
            .map_err(QueryError::from)
    }

    async fn delete(&self, project_id: &str, value: &str) -> Result<(), QueryError> {
        const DELETE_QUERY: &'static str = r#"
            DELETE FROM projects_search
            WHERE project_id = $1 AND value = $2
        "#;

        let mut conn = self.connection_pool
            .get()
            .await?;

        sqlx::query(DELETE_QUERY)
            .bind(project_id)
            .bind(value)
            .execute(conn.as_mut())
            .await
            .map(|_| ())
            .map_err(QueryError::from)
    }

    async fn query(&self, terms: Vec<&str>) -> Result<Vec<SearchRecord>, QueryError> {
        const SEARCH_QUERY: &'static str = r#"
            SELECT s.project_id as pid, s.name as n, s.value <-> q.value AS s
            FROM UNNEST($1) as q(value)
            JOIN projects_search s 
            ON s.value % q.value
        "#;

        let mut conn = self.connection_pool
            .get()
            .await?;

        let mut results = sqlx::query(SEARCH_QUERY)
            .bind(terms)
            .map(|row: PgRow| (
                row.get::<String, &str>("pid"),
                row.get("n"),
                1. - row.get::<f32, &str>("s"),
            ))
            .fetch(conn.as_mut());

        let mut rows = HashMap::new();

        while let Some((project_id, name, score)) = results.try_next().await? {
            rows.entry(project_id.to_owned())
                .or_insert_with(|| SearchRecord {
                    project_id,
                    name,
                    score: 0.0,
                })
                .score += score
        }

        let mut rows: Vec<SearchRecord> = rows.into_values().collect();

        rows.sort_by(|l, r| r.score.total_cmp(&l.score));
        rows.truncate(32);

        Ok(rows)
    }
}