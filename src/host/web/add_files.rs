use auth::client::axum::extractors::Authenticate;
use axum::Json;
use axum::{extract::Path, response::IntoResponse};
use or_status_code::{OrInternalServerError, OrNotFound};
use rand::distributions::{Alphanumeric, DistString};
use serde::Deserialize;
use axum::http::StatusCode;

use crate::host::axum::extractors::{events_repository::EventsRepositoryExtractor, snapshots_repository::SnapshotsRepositoryExtractor};
use crate::host::events::{AddFilesEvent, FileMap};
use crate::host::repository::snapshots::SnapshotsRepository;
use crate::host::repository::events::EventsRepository;

use super::ApiResult;

#[derive(Deserialize)]
pub struct AddFilesRequest {
    pub files: Vec<AddFileRequest>,
}

#[derive(Deserialize)]
pub struct AddFileRequest {
    pub path: String,
    pub file_id: String,
}

pub async fn add_files(
    Authenticate(user): Authenticate,
    snapshots_repository: SnapshotsRepositoryExtractor,
    events_repository: EventsRepositoryExtractor,
    Path(project_id): Path<String>,
    Json(request): Json<AddFilesRequest>,
) -> ApiResult<impl IntoResponse> {
    let mut project = snapshots_repository
        .get_by_id(&project_id, "latest")
        .await
        .or_internal_server_error()?
        .or_not_found()?;

    if project.user_id != user.id {
        return Err(StatusCode::FORBIDDEN);
    }

    let event = AddFilesEvent {
        event_id: Alphanumeric.sample_string(&mut rand::thread_rng(), 64),
        files: request.files
            .into_iter()
            .map(|file_request| FileMap {
                path: file_request.path,
                file_id: file_request.file_id,
            })
            .collect()
    };

    events_repository
        .create(&project.id, event.clone())
        .await
        .or_internal_server_error()?;

    project.apply_event(event);

    snapshots_repository
        .delete(&project_id, "latest")
        .await
        .or_internal_server_error()?;

    snapshots_repository
        .create(&project_id, "latest", project)
        .await
        .or_internal_server_error()?;

    Ok(StatusCode::NO_CONTENT)
}