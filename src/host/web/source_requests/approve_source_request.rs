use auth::client::axum::extractors::{Authenticate, ClaimsUser};
use axum::{extract::Path, response::IntoResponse};
use or_status_code::{OrInternalServerError, OrNotFound};
use axum::http::StatusCode;

use crate::host::{axum::extractors::{snapshots_repository::SnapshotsRepositoryExtractor, source_request_repository::SourceRequestsRepositoryExtractor}, web::ApiResult};
use crate::host::repository::snapshots::SnapshotsRepository;
use crate::host::repository::source_requests::SourceRequestRepository;

pub async fn approve_source_request(
    Authenticate(user): Authenticate<ClaimsUser>,
    snapshots_repository: SnapshotsRepositoryExtractor,
    source_request_repository: SourceRequestsRepositoryExtractor,
    Path((project_id, source_request_id)): Path<(String, i32)>,
) -> ApiResult<impl IntoResponse> {
    let project = snapshots_repository
        .get_by_id(&project_id, "latest")
        .await
        .or_internal_server_error()?
        .or_not_found()?;

    if project.user_id != user.id {
        return Err(StatusCode::FORBIDDEN);
    }

    let source_request = source_request_repository
        .get_approvable(source_request_id)
        .await
        .or_internal_server_error()?
        .or_not_found()?;

    let approved = source_request.approve(user.id);

    source_request_repository
        .update(source_request_id, approved)
        .await
        .or_internal_server_error()?;

    Ok(StatusCode::NO_CONTENT)
}