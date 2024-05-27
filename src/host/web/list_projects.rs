use axum::Json;
use axum::{extract::Query, response::IntoResponse};
use or_status_code::OrInternalServerError;
use serde::{Deserialize, Serialize};

use crate::host::axum::extractors::snapshots_repository::SnapshotsRepositoryExtractor;
use crate::host::repository::snapshots::{self, SnapshotsRepository};

use super::ApiResult;

#[derive(Deserialize)]
pub struct ListProjectsQuery {
    #[serde(rename = "r")]
    #[serde(default = "default_ranking")]
    pub ranking: Ranking,
    #[serde(rename = "p")]
    #[serde(default = "default_period")]
    pub period: Period,
    #[serde(rename = "l")]
    #[serde(default = "default_limit")]
    pub limit: u8,
}

#[derive(Deserialize)]
pub enum Ranking {
    #[serde(rename = "p")]
    Popularity,
}

impl Into<snapshots::Ranking> for Ranking {
    fn into(self) -> snapshots::Ranking {
        match self {
            Ranking::Popularity => snapshots::Ranking::ViewCount,
        }
    }
}

#[derive(Deserialize)]
pub enum Period {
    #[serde(rename = "a")]
    All,
}

impl Into<snapshots::Period> for Period {
    fn into(self) -> snapshots::Period {
        match self {
            Period::All => snapshots::Period::All,
        }
    }
}

fn default_ranking() -> Ranking {
    Ranking::Popularity
}

fn default_period() -> Period {
    Period::All
}

fn default_limit() -> u8 {
    10
}

#[derive(Serialize)]
pub struct ListProjectsResponse {
    #[serde(rename = "p")]
    projects: Vec<ProjectResponse>,
}

#[derive(Serialize)]
pub struct ProjectResponse {
    #[serde(rename = "id")]
    id: String,
    #[serde(rename = "n")]
    name: String,
}

pub async fn list_projects(
    snapshots_repository: SnapshotsRepositoryExtractor,
    Query(query): Query<ListProjectsQuery>
) -> ApiResult<impl IntoResponse> {
    let projects = snapshots_repository
        .list(query.ranking, query.period, query.limit as i32)
        .await
        .or_internal_server_error()?;

    let response_body = ListProjectsResponse {
        projects: projects
            .into_iter()
            .map(|project| ProjectResponse {
                id: project.id,
                name: project.name,
            })
            .collect(),
    };

    Ok(Json(response_body))
}