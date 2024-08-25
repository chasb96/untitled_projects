use auth_client::axum::extractors::{Authenticate, ClaimsUser};
use axum::extract::Path;
use axum::response::IntoResponse;
use axum_extra::protobuf::Protobuf;
use or_status_code::{OrInternalServerError, OrNotFound};
use prost::Message;
use axum::http::StatusCode;
use rand::distributions::{Alphanumeric, DistString};

use crate::web::ApiResult;
use crate::repository::source_requests::comments::CreateSourceRequestComment;
use crate::axum::extractors::source_request_repository::SourceRequestsRepositoryExtractor;
use crate::axum::extractors::source_request_comments_repository::SourceRequestCommentsRepositoryExtractor;
use crate::repository::source_requests::SourceRequestRepository;
use crate::repository::source_requests::comments::SourceRequestCommentRepository;

#[derive(Message)]
pub struct CreateCommentRequest {
    #[prost(string, tag = "1")]
    pub content: String,
}

#[derive(Message)]
pub struct CommentResponse {
    #[prost(string, tag = "1")]
    pub id: String,
}

pub async fn create_source_request_comment(
    Authenticate(user): Authenticate<ClaimsUser>,
    source_request_repository: SourceRequestsRepositoryExtractor,
    comments_repository: SourceRequestCommentsRepositoryExtractor,
    Path((project_id, source_request_id)): Path<(String, String)>,
    Protobuf(request): Protobuf<CreateCommentRequest>,
) -> ApiResult<impl IntoResponse> {
    let source_request = source_request_repository
        .get_by_id(&source_request_id)
        .await
        .or_internal_server_error()?
        .or_not_found()?;

    if source_request.project_id() != project_id {
        return Err(StatusCode::NOT_FOUND);
    }

    let comment_id = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);

    let comment = CreateSourceRequestComment {
        id: &comment_id,
        source_request_id: &source_request_id,
        user_id: &user.id,
        content: &request.content,
    };

    comments_repository
        .create(comment)
        .await
        .or_internal_server_error()?;

    Ok(Protobuf(CommentResponse {
        id: comment_id
    }))
}