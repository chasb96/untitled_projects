use auth_client::axum::extractors::Authenticate;
use auth_client::axum::extractors::ClaimsUser;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_extra::protobuf::Protobuf;
use or_status_code::OrInternalServerError;
use or_status_code::OrStatusCode;
use prost::Message;

use crate::axum::extractors::message_queue::MessageQueueExtractor;
use crate::axum::extractors::snapshots_repository::SnapshotsRepositoryExtractor;
use crate::message_queue::ProjectViewed;
use crate::repository::snapshots::SnapshotsRepository;

use super::ApiResult;

#[derive(Message)]
pub struct VersionResponse {
    #[prost(string, tag = "1")]
    pub id: String,
    #[prost(string, tag = "2")]
    pub name: String,
    #[prost(string, tag = "3")]
    pub user_id: String,
    #[prost(string, tag = "4")]
    pub event_id: String,
    #[prost(message, repeated, tag = "5")]
    pub files: Vec<VersionFileReponse>,
}

#[derive(Message)]
pub struct VersionFileReponse {
    #[prost(string, tag = "1")]
    pub id: String,
    #[prost(string, tag = "2")]
    pub name: String,
}

pub async fn get_version_by_id(
    Authenticate(user): Authenticate<Option<ClaimsUser>>,
    message_queue: MessageQueueExtractor,
    snapshots_repository: SnapshotsRepositoryExtractor,
    Path((project_id, version)): Path<(String, String)>
) -> ApiResult<impl IntoResponse> {
    let project = snapshots_repository
        .get_by_id(&project_id, &version)
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

    let response_body = VersionResponse {
        id: project.id,
        name: project.name,
        user_id: project.user_id,
        event_id: project.event_id,
        files: project.files
            .into_iter()
            .map(|file| VersionFileReponse {
                id: file.1,
                name: file.0,
            })
            .collect(),
    };

    Ok(Protobuf(response_body))
}