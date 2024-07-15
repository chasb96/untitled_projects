use axum::{extract::Path, response::IntoResponse, Json};
use or_status_code::{OrInternalServerError, OrNotFound};
use axum::http::StatusCode;
use serde::Serialize;

use crate::{axum::extractors::{source_request_comments_repository::SourceRequestCommentsRepositoryExtractor, source_request_repository::SourceRequestsRepositoryExtractor}, web::ApiResult};
use crate::repository::source_requests::SourceRequestRepository;
use crate::repository::source_requests::comments::SourceRequestCommentRepository;

#[derive(Serialize)]
pub struct ListCommentsResponse {
    #[serde(rename = "c")]
    comments: Vec<CommentResponse>,
}

#[derive(Serialize)]
pub struct CommentResponse {
    #[serde(rename = "i")]
    id: i32,
    #[serde(rename = "u")]
    user_id: String,
    #[serde(rename = "c")]
    content: String,
}

pub async fn list_source_request_comments(
    source_request_repository: SourceRequestsRepositoryExtractor,
    comments_repository: SourceRequestCommentsRepositoryExtractor,
    Path((project_id, source_request_id)): Path<(String, String)>,
) -> ApiResult<impl IntoResponse> {
    let source_request = source_request_repository
        .get_by_id(&source_request_id)
        .await
        .or_internal_server_error()?
        .or_not_found()?;

    if source_request.project_id() != project_id {
        return Err(StatusCode::NOT_FOUND);
    }

    let comments = comments_repository
        .list(&source_request_id)
        .await
        .or_internal_server_error()?;

    let response = ListCommentsResponse {
        comments: comments
            .into_iter()
            .map(|comment| CommentResponse {
                id: comment.id,
                user_id: comment.user_id,
                content: comment.content,
            })
            .collect(),
    };

    Ok(Json(response))
}