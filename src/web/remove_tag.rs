use auth_client::axum::extractors::{Authenticate, ClaimsUser};
use axum::{extract::Path, response::IntoResponse};
use or_status_code::{OrInternalServerError, OrNotFound};
use axum::http::StatusCode;

use crate::axum::extractors::message_queue::MessageQueueExtractor;
use crate::axum::extractors::snapshots_repository::SnapshotsRepositoryExtractor;
use crate::message_queue::RemoveTag;
use crate::repository::snapshots::SnapshotsRepository;

use super::ApiResult;

pub async fn remove_tag(
    Authenticate(user): Authenticate<ClaimsUser>,
    snapshots_repository: SnapshotsRepositoryExtractor,
    message_queue: MessageQueueExtractor,
    Path((project_id, tag)): Path<(String, String)>,
) -> ApiResult<impl IntoResponse> {
    let project = snapshots_repository
        .get_by_id(&project_id, "latest")
        .await
        .or_internal_server_error()?
        .or_not_found()?;

    if project.user_id != user.id {
        return Err(StatusCode::FORBIDDEN);
    }

    message_queue
        .send(RemoveTag {
            project_id,
            tag: tag.to_lowercase(),
        })
        .await;

    Ok(StatusCode::ACCEPTED)
}