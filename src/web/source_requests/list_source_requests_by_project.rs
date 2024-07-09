use axum::extract::Path;
use axum::response::IntoResponse;
use axum::Json;
use or_status_code::OrInternalServerError;

use crate::axum::extractors::source_request_repository::SourceRequestsRepositoryExtractor;
use crate::repository::source_requests::SourceRequestRepository;
use crate::web::ApiResult;

use super::{ListSourceRequestItem, ListSourceRequests, SourceRequestSummary};

pub async fn list_source_requests_by_project(
    source_requests_repository: SourceRequestsRepositoryExtractor,
    Path(project_id): Path<String>,
) -> ApiResult<impl IntoResponse> {
    let source_requests = source_requests_repository
        .list_by_project_id(&project_id)
        .await
        .or_internal_server_error()?;

    let response = ListSourceRequests {
        source_requests: source_requests
            .into_iter()
            .map(|(id, source_request_summary)| ListSourceRequestItem {
                id,
                source_request: SourceRequestSummary::from(source_request_summary)
            })
            .collect(),
    };

    Ok(Json(response))
}