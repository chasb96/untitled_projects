use auth_client::axum::extractors::{Authenticate, ClaimsUser};
use axum::{extract::Path, response::IntoResponse};
use axum_extra::protobuf::Protobuf;
use or_status_code::{OrBadRequest, OrInternalServerError, OrNotFound};
use axum::http::StatusCode;

use crate::{axum::extractors::{events_repository::EventsRepositoryExtractor, message_queue::MessageQueueExtractor, snapshots_repository::SnapshotsRepositoryExtractor}, events::EventKind, message_queue::CreateSnapshot};
use crate::repository::snapshots::SnapshotsRepository;
use crate::repository::events::EventsRepository;

use super::{events::{EventRequest, EventRequestMessage}, validate::Validate, ApiResult};

pub async fn event(
    Authenticate(user): Authenticate<ClaimsUser>,
    snapshots_repository: SnapshotsRepositoryExtractor,
    events_repository: EventsRepositoryExtractor,
    message_queue: MessageQueueExtractor,
    Path(project_id): Path<String>,
    Protobuf(request): Protobuf<EventRequestMessage>,
) -> ApiResult<impl IntoResponse> {
    let request: EventRequest = request.try_into().or_bad_request()?;

    request.validate().or_bad_request()?;

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