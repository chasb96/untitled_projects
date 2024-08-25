use auth_client::axum::extractors::{Authenticate, ClaimsUser};
use axum::response::IntoResponse;
use axum::extract::Path;
use axum_extra::protobuf::Protobuf;
use or_status_code::{OrInternalServerError, OrNotFound};
use prost::Message;
use rand::distributions::{Alphanumeric, DistString};

use crate::repository::source_requests::{CreateNewSourceRequest, NewFileMap};
use crate::axum::extractors::source_request_repository::SourceRequestsRepositoryExtractor;
use crate::axum::extractors::snapshots_repository::SnapshotsRepositoryExtractor;
use crate::web::ApiResult;
use crate::repository::snapshots::SnapshotsRepository;
use crate::repository::source_requests::SourceRequestRepository;

#[derive(Message)]
pub struct CreateSourceRequestRequest {
    #[prost(string, tag = "1")]
    pub title: String,
    #[prost(string, tag = "2")]
    pub description: String,
    #[prost(message, repeated, tag = "3")]
    pub files: Vec<FileMap>,
}

#[derive(Message)]
pub struct FileMap {
    #[prost(string, tag = "1")]
    pub path: String,
    #[prost(string, tag = "2")]
    pub file_id: String,
}

#[derive(Message)]
pub struct CreateSourceRequestResponse {
    #[prost(string, tag = "1")]
    pub id: String,
}

pub async fn create_source_request(
    Authenticate(user): Authenticate<ClaimsUser>,
    snapshots_repository: SnapshotsRepositoryExtractor,
    source_request_repository: SourceRequestsRepositoryExtractor,
    Path(project_id): Path<String>,
    Protobuf(request): Protobuf<CreateSourceRequestRequest>,
) -> ApiResult<impl IntoResponse> {
    let project = snapshots_repository
        .get_by_id(&project_id, "latest")
        .await
        .or_internal_server_error()?
        .or_not_found()?;

    let source_request_id = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);

    let source_request = CreateNewSourceRequest {
        project_id: &project.id,
        user_id: &user.id,
        title: &request.title,
        description: &request.description,
        files: request
            .files
            .iter()
            .map(|file| NewFileMap {
                path: &file.path,
                file_id: &file.file_id,
            })
            .collect(),
    };

    source_request_repository
        .create(&source_request_id, source_request)
        .await
        .or_internal_server_error()?;

    Ok(Protobuf(CreateSourceRequestResponse {
        id: source_request_id,
    }))
}