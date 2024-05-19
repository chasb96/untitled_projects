use auth::client::axum::extractors::Authenticate;
use axum::{extract::Path, response::IntoResponse, Json};
use or_status_code::{OrInternalServerError, OrNotFound};
use rand::distributions::{Alphanumeric, DistString};
use serde::Deserialize;
use axum::http::StatusCode;

use crate::host::{axum::extractors::{events_repository::EventsRepositoryExtractor, snapshots_repository::SnapshotsRepositoryExtractor}, events::RemoveFilesEvent, repository::events::EventsRepository};
use crate::host::repository::snapshots::SnapshotsRepository;

use super::ApiResult;

#[derive(Deserialize)]
pub struct RemoveFilesRequest {
    pub paths: Vec<String>,
}

pub async fn remove_files(
    Authenticate(user): Authenticate,
    snapshots_repository: SnapshotsRepositoryExtractor,
    events_repository: EventsRepositoryExtractor,
    Path(project_id): Path<String>,
    Json(request): Json<RemoveFilesRequest>,
) -> ApiResult<impl IntoResponse> {
    let mut project = snapshots_repository
        .get_by_id(&project_id, "latest")
        .await
        .or_internal_server_error()?
        .or_not_found()?;

    if project.user_id != user.id {
        return Err(StatusCode::FORBIDDEN);
    }

    let event = RemoveFilesEvent {
        event_id: Alphanumeric.sample_string(&mut rand::thread_rng(), 64),
        paths: request.paths,
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

    Ok(StatusCode::OK)
}