use auth_client::axum::extractors::{Authenticate, ClaimsUser};
use axum::{extract::{Path, Query}, http::{HeaderMap, StatusCode}, response::IntoResponse};
use json_or_protobuf::JsonOrProtobuf;
use or_status_code::{OrInternalServerError, OrStatusCode};
use serde::Deserialize;

use crate::{axum::extractors::{message_queue::MessageQueueExtractor, snapshots_repository::SnapshotsRepositoryExtractor}, message_queue::ProjectViewed};
use crate::repository::snapshots::SnapshotsRepository;

use super::ApiResult;

use prost::Message;
use serde::Serialize;

#[derive(Serialize, Message)]
pub struct ProjectResponse {
    #[serde(rename = "id")]
    #[prost(string, tag = "1")]
    pub id: String,
    #[serde(rename = "n")]
    #[prost(string, tag = "2")]
    pub name: String,
    #[serde(rename = "uid")]
    #[prost(string, tag = "3")]
    pub user_id: String,
    #[serde(rename = "f")]
    #[prost(message, repeated, tag = "4")]
    pub files: Vec<ProjectFileReponse>,
}

#[derive(Serialize, Message)]
pub struct ProjectFileReponse {
    #[serde(rename = "id")]
    #[prost(string, tag = "1")]
    pub id: String,
    #[serde(rename = "n")]
    #[prost(string, tag = "2")]
    pub name: String,
}

#[derive(Deserialize)]
pub struct GetProjectByIdQuery {
    pub version: Option<String>
}

pub async fn get_project_by_id(
    Authenticate(user): Authenticate<Option<ClaimsUser>>,
    message_queue: MessageQueueExtractor,
    snapshots_repository: SnapshotsRepositoryExtractor,
    headers: HeaderMap,
    Path(id): Path<String>,
    query: Query<GetProjectByIdQuery>
) -> ApiResult<impl IntoResponse> {
    let version = match &query.version {
        Some(version) => version,
        None => "latest"
    };

    let project = snapshots_repository
        .get_by_id(&id, version)
        .await
        .or_internal_server_error()?
        .or_status_code(StatusCode::NOT_FOUND)?;

    if user.is_none() || user.unwrap().id != project.user_id {
        message_queue
            .send(ProjectViewed { 
                id: project.id.clone()
            })
            .await;
    }

    let response_body = ProjectResponse {
        id: project.id,
        name: project.name,
        user_id: project.user_id,
        files: project.files
            .into_iter()
            .map(|file| ProjectFileReponse {
                id: file.1,
                name: file.0,
            })
            .collect(),
    };

    Ok(JsonOrProtobuf::from_accept_header(response_body, &headers))
}