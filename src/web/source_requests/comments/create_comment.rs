use auth::client::axum::extractors::{Authenticate, ClaimsUser};
use axum::{extract::Path, response::IntoResponse, Json};
use chrono::Utc;
use or_status_code::{OrInternalServerError, OrNotFound};
use serde::{Deserialize, Serialize};
use axum::http::StatusCode;

use crate::{axum::extractors::{source_request_comments_repository::SourceRequestCommentsRepositoryExtractor, source_request_repository::SourceRequestsRepositoryExtractor, validate::Validated}, repository::source_requests::comments::CreateSourceRequestComment, web::{validate::{Validate, ValidationError}, ApiResult}};
use crate::repository::source_requests::SourceRequestRepository;
use crate::repository::source_requests::comments::SourceRequestCommentRepository;

#[derive(Deserialize)]
pub struct CreateCommentRequest {
    #[serde(rename = "c")]
    pub content: String,
}

impl Validate for CreateCommentRequest {
    fn validate(&self) -> Result<(), ValidationError> {
        let content_len = self.content.len();

        if content_len == 0 { return Err("Content must have atleast one character".into()) }

        Ok(())
    }
}

#[derive(Serialize)]
pub struct CommentResponse {
    pub id: i32,
}

pub async fn create_source_request_comment(
    Authenticate(user): Authenticate<ClaimsUser>,
    source_request_repository: SourceRequestsRepositoryExtractor,
    comments_repository: SourceRequestCommentsRepositoryExtractor,
    Path((project_id, source_request_id)): Path<(String, i32)>,
    Validated(Json(request)): Validated<Json<CreateCommentRequest>>,
) -> ApiResult<impl IntoResponse> {
    let source_request = source_request_repository
        .get_by_id(source_request_id)
        .await
        .or_internal_server_error()?
        .or_not_found()?;

    if source_request.project_id() != project_id {
        return Err(StatusCode::NOT_FOUND);
    }

    let comment = CreateSourceRequestComment {
        source_request_id,
        user_id: &user.id,
        content: &request.content,
        created_at: &Utc::now().naive_utc(),
    };

    let comment_id = comments_repository
        .create(comment)
        .await
        .or_internal_server_error()?;

    Ok(Json(CommentResponse {
        id: comment_id
    }))
}