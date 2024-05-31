use auth::client::axum::extractors::{Authenticate, ClaimsUser};
use axum::{extract::Path, response::IntoResponse, Json};
use or_status_code::{OrInternalServerError, OrNotFound};
use axum::http::StatusCode;

use crate::host::{axum::extractors::{events_repository::EventsRepositoryExtractor, message_queue::MessageQueueExtractor, snapshots_repository::SnapshotsRepositoryExtractor, validate::Validated}, events::EventKind, message_queue::CreateSnapshot};
use crate::host::repository::snapshots::SnapshotsRepository;
use crate::host::repository::events::EventsRepository;

use super::{events::EventRequest, ApiResult};

pub async fn event(
    Authenticate(user): Authenticate<ClaimsUser>,
    snapshots_repository: SnapshotsRepositoryExtractor,
    events_repository: EventsRepositoryExtractor,
    message_queue: MessageQueueExtractor,
    Path(project_id): Path<String>,
    Validated(Json(request)): Validated<Json<EventRequest>>,
) -> ApiResult<impl IntoResponse> {
    let mut project = snapshots_repository
        .get_by_id(&project_id, "latest")
        .await
        .or_internal_server_error()?
        .or_not_found()?;

    if project.user_id != user.id {
        return Err(StatusCode::FORBIDDEN);
    }

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