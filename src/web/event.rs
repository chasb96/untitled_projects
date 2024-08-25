use axum::{extract::Path, response::IntoResponse, Json};
use axum::http::StatusCode;
use or_status_code::{OrInternalServerError, OrNotFound};

use crate::{axum::extractors::{events_repository::EventsRepositoryExtractor, message_queue::MessageQueueExtractor, snapshots_repository::SnapshotsRepositoryExtractor}, events::EventKind, message_queue::CreateSnapshot};
use crate::repository::snapshots::SnapshotsRepository;
use crate::repository::events::EventsRepository;

use super::{events::EventRequest, ApiResult};

pub async fn event(
    snapshots_repository: SnapshotsRepositoryExtractor,
    events_repository: EventsRepositoryExtractor,
    message_queue: MessageQueueExtractor,
    Path(project_id): Path<String>,
    Json(request): Json<EventRequest>,
) -> ApiResult<impl IntoResponse> {
    let mut project = snapshots_repository
        .get_by_id(&project_id, "latest")
        .await
        .or_internal_server_error()?
        .or_not_found()?;

    let event: EventKind = request.into();

    events_repository
        .create(&project.id, event.clone())
        .await
        .or_internal_server_error()?;

    project.apply_event(event);

    message_queue
        .send(CreateSnapshot {
            project_id: project.id.clone(),
            version: "latest".to_string(),
            snapshot: project,
        })
        .await;

    Ok(StatusCode::NO_CONTENT)
}