use axum::{extract::Path, response::IntoResponse};
use axum_extra::protobuf::Protobuf;
use or_status_code::{OrInternalServerError, OrNotFound};
use axum::http::StatusCode;
use prost::Message;

use crate::{axum::extractors::{source_request_comments_repository::SourceRequestCommentsRepositoryExtractor, source_request_repository::SourceRequestsRepositoryExtractor}, web::ApiResult};
use crate::repository::source_requests::SourceRequestRepository;
use crate::repository::source_requests::comments::SourceRequestCommentRepository;

#[derive(Message)]
pub struct ListCommentsResponse {
    #[prost(message, repeated, tag = "1")]
    comments: Vec<CommentResponse>,
}

#[derive(Message)]
pub struct CommentResponse {
    #[prost(string, tag = "1")]
    id: String,
    #[prost(string, tag = "2")]
    user_id: String,
    #[prost(string, tag = "3")]
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

    Ok(Protobuf(response))
}