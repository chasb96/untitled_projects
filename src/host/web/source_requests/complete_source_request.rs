use auth::client::axum::extractors::{Authenticate, ClaimsUser};
use axum::{extract::Path, response::IntoResponse};
use or_status_code::{OrInternalServerError, OrNotFound};
use axum::http::StatusCode;

use crate::host::{axum::extractors::{events_repository, message_queue::MessageQueueExtractor, snapshots_repository::SnapshotsRepositoryExtractor, source_request_repository::SourceRequestsRepositoryExtractor}, events::EventKind, message_queue::CreateSnapshot, web::ApiResult};
use crate::host::repository::snapshots::SnapshotsRepository;
use crate::host::repository::source_requests::SourceRequestRepository;
use crate::host::repository::events::EventsRepository;

pub async fn complete_source_request(
    Authenticate(user): Authenticate<ClaimsUser>,
    snapshots_repository: SnapshotsRepositoryExtractor,
    source_request_repository: SourceRequestsRepositoryExtractor,
    events_repository: events_repository::EventsRepositoryExtractor,
    message_queue: MessageQueueExtractor,
    Path((project_id, source_request_id)): Path<(String, i32)>,
) -> ApiResult<impl IntoResponse> {
    let mut project = snapshots_repository
        .get_by_id(&project_id, "latest")
        .await
        .or_internal_server_error()?
        .or_not_found()?;

    if project.user_id != user.id {
        return Err(StatusCode::FORBIDDEN);
    }

    let source_request = source_request_repository
        .get_completable(source_request_id)
        .await
        .or_internal_server_error()?
        .or_not_found()?;

    let completed = source_request.complete();

    source_request_repository
        .update(source_request_id, completed.clone())
        .await
        .or_internal_server_error()?;

    let event: EventKind = completed.into();

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