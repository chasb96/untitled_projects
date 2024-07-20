use axum::http::HeaderMap;
use axum::{extract::Query, response::IntoResponse};
use json_or_protobuf::JsonOrProtobuf;
use or_status_code::OrInternalServerError;
use prost::Message;
use serde::{Deserialize, Serialize};

use crate::axum::extractors::snapshots_repository::SnapshotsRepositoryExtractor;
use crate::repository::snapshots::{ListQuery, SnapshotsRepository};

use super::ApiResult;

#[derive(Deserialize)]
#[serde(untagged)]
pub enum ListProjectsQuery {
    ProjectIds { 
        #[serde(rename = "pids")]
        project_ids: Vec<String> 
    },
    UserId { 
        #[serde(rename = "uid")]
        user_id: String 
    },
}

#[derive(Serialize, Message)]
pub struct ListProjectsResponse {
    #[serde(rename = "p")]
    #[prost(message, repeated, tag = "1")]
    projects: Vec<ProjectsResponse>,
}

#[derive(Serialize, Message)]
pub struct ProjectsResponse {
    #[serde(rename = "id")]
    #[prost(string, tag = "1")]
    id: String,
    #[serde(rename = "n")]
    #[prost(string, tag = "2")]
    name: String,
}

pub async fn list_projects(
    snapshots_repository: SnapshotsRepositoryExtractor,
    headers: HeaderMap,
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

    Ok(JsonOrProtobuf::from_accept_header(response_body, &headers))
}

impl Into<ListQuery> for ListProjectsQuery {
    fn into(self) -> ListQuery {
        match self {
            ListProjectsQuery::ProjectIds { project_ids } => ListQuery::ProjectIds { project_ids },
            ListProjectsQuery::UserId { user_id } => ListQuery::UserId { user_id },
        }
    }
}