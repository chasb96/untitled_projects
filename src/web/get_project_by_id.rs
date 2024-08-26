use auth_client::axum::extractors::{Authenticate, ClaimsUser};
use axum::{extract::{Path, Query}, http::StatusCode, response::IntoResponse};
use axum_extra::protobuf::Protobuf;
use or_status_code::{OrInternalServerError, OrStatusCode};
use serde::Deserialize;

use crate::{axum::extractors::{message_queue::MessageQueueExtractor, snapshots_repository::SnapshotsRepositoryExtractor}, message_queue::ProjectViewed};
use crate::repository::snapshots::SnapshotsRepository;

use super::ApiResult;

use prost::Message;

#[derive(Message)]
pub struct ProjectResponse {
    #[prost(string, tag = "1")]
    pub id: String,
    #[prost(string, tag = "2")]
    pub name: String,
    #[prost(string, tag = "3")]
    pub user_id: String,
    #[prost(string, tag = "4")]
    pub event_id: String,
    #[prost(message, repeated, tag = "5")]
    pub files: Vec<ProjectFileReponse>,
}

#[derive(Message)]
pub struct ProjectFileReponse {
    #[prost(string, tag = "1")]
    pub id: String,
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
        event_id: project.event_id,
        files: project.files
            .into_iter()
            .map(|file| ProjectFileReponse {
                id: file.1,
                name: file.0,
            })
            .collect(),
    };

    Ok(Protobuf(response_body))
}