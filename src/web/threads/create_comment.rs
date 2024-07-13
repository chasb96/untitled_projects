use auth_client::axum::extractors::{Authenticate, ClaimsUser};
use axum::{extract::Path, response::IntoResponse, Json};
use axum::http::StatusCode;
use chrono::Utc;
use or_status_code::OrInternalServerError;
use serde::{Deserialize, Serialize};

use crate::{axum::extractors::{threads_repository::ThreadsRepositoryExtractor, validate::Validated}, repository::threads::NewComment, web::{validate::{Validate, ValidationError}, ApiResult}};
use crate::repository::threads::ThreadsRepository;

#[derive(Deserialize)]
pub struct CreateProjectRequest {
    #[serde(rename = "c")]
    pub content: String,
}

impl Validate for CreateProjectRequest {
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

pub async fn create_comment(
    Authenticate(user): Authenticate<ClaimsUser>,
    threads_repository: ThreadsRepositoryExtractor,
    Path((project_id, thread_id)): Path<(String, i32)>,
    Validated(Json(request)): Validated<Json<CreateProjectRequest>>,
) ->ApiResult<impl IntoResponse> {
    let thread = threads_repository
        .get_by_id(thread_id)
        .await
        .or_internal_server_error()?;

    let thread = match thread {
        None => return Err(StatusCode::NOT_FOUND),
        Some(thread) if thread.project_id != project_id => return Err(StatusCode::NOT_FOUND),
        Some(thread) => thread,
    };

    let comment_id = threads_repository
        .create_comment(NewComment {
            thread_id: thread.id,
            user_id: &user.id,
            content: &request.content,
            created_at: &Utc::now().naive_utc(),
        })
        .await
        .or_internal_server_error()?;

    Ok(Json(CommentResponse {
        id: comment_id
    }))
}