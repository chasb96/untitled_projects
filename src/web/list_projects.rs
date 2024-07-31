use axum::{extract::Query, response::IntoResponse};
use axum_extra::protobuf::Protobuf;
use or_status_code::OrInternalServerError;
use prost::Message;
use serde::Deserialize;

use crate::axum::extractors::snapshots_repository::SnapshotsRepositoryExtractor;
use crate::repository::snapshots::{ListQuery, SnapshotsRepository};

use super::ApiResult;

#[derive(Deserialize)]
#[serde(untagged)]
pub enum ListProjectsQuery {
    ProjectIds { 
        #[serde(rename = "pids", deserialize_with = "filter_vec_deserlialize")]
        project_ids: Vec<String> 
    },
    UserId { 
        #[serde(rename = "uid")]
        user_id: String 
    },
}

fn filter_vec_deserlialize<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let project_ids = String::deserialize(deserializer)?
        .split(',')
        .map(|str| str.to_string()) 
        .collect();

    Ok(project_ids)
}

#[derive(Message)]
pub struct ListProjectsResponse {
    #[prost(message, repeated, tag = "1")]
    projects: Vec<ProjectsResponse>,
}

#[derive(Message)]
pub struct ProjectsResponse {
    #[prost(string, tag = "1")]
    id: String,
    #[prost(string, tag = "2")]
    name: String,
}

pub async fn list_projects(
    snapshots_repository: SnapshotsRepositoryExtractor,
    Query(query): Query<ListProjectsQuery>,
) -> ApiResult<impl IntoResponse> {
    let snapshots = snapshots_repository
        .list(&query.into())
        .await
        .or_internal_server_error()?;

    let response_body = ListProjectsResponse {
        projects: snapshots
            .into_iter()
            .map(|snapshot| ProjectsResponse {
                id: snapshot.id,
                name: snapshot.name,
            })
            .collect(),
    };

    Ok(Protobuf(response_body))
}

impl Into<ListQuery> for ListProjectsQuery {
    fn into(self) -> ListQuery {
        match self {
            ListProjectsQuery::ProjectIds { project_ids } => ListQuery::ProjectIds { project_ids },
            ListProjectsQuery::UserId { user_id } => ListQuery::UserId { user_id },
        }
    }
}