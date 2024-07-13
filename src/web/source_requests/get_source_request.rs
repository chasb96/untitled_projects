use axum::extract::Path;
use axum::response::IntoResponse;
use axum::Json;
use or_status_code::{OrInternalServerError, OrNotFound};

use crate::axum::extractors::source_request_repository::SourceRequestsRepositoryExtractor;
use crate::{axum::extractors::snapshots_repository::SnapshotsRepositoryExtractor, web::ApiResult};
use crate::repository::snapshots::SnapshotsRepository;
use crate::repository::source_requests::SourceRequestRepository;

use super::SourceRequest;

pub async fn get_source_request(
    snapshots_repository: SnapshotsRepositoryExtractor,
    source_request_repository: SourceRequestsRepositoryExtractor,
    Path((project_id, source_request_id)): Path<(String, String)>,
) -> ApiResult<impl IntoResponse> {
    _ = snapshots_repository
        .get_by_id(&project_id, "latest")
        .await
        .or_internal_server_error()?
        .or_not_found()?;

    let source_request = source_request_repository
        .get_by_id(&source_request_id)
        .await
        .or_internal_server_error()?
        .or_not_found()?;

    let response: SourceRequest = source_request.into();

    Ok(Json(response))
}