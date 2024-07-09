use axum::{extract::Path, response::IntoResponse, Json};
use or_status_code::OrInternalServerError;
use serde::Serialize;
use axum::http::StatusCode;

use crate::{axum::extractors::threads_repository::ThreadsRepositoryExtractor, web::ApiResult};
use crate::repository::threads::ThreadsRepository;

#[derive(Serialize)]
pub struct ThreadResponse {
    pub id: i32,
    #[serde(rename = "u")]
    pub user_id: String,
    #[serde(rename = "t")]
    pub title: String,
    #[serde(rename = "cr")]
    pub created_at: String,
    #[serde(rename = "c")]
    pub comments: Vec<CommentResponse>,
}

#[derive(Serialize)]
pub struct CommentResponse {
    pub id: i32,
    #[serde(rename = "u")]
    pub user_id: String,
    #[serde(rename = "c")]
    pub content: String,
    #[serde(rename = "cr")]
    pub created_at: String,
}

pub async fn get_thread_by_id(
    threads_repository: ThreadsRepositoryExtractor,
    Path((project_id, thread_id)): Path<(String, i32)>,
) -> ApiResult<impl IntoResponse> {
    let thread = threads_repository
        .get_by_id(thread_id)
        .await
        .or_internal_server_error()?;

    let thread = match thread {
        None => return Err(StatusCode::NOT_FOUND),
        Some(thread) if thread.project_id != project_id => return Err(StatusCode::NOT_FOUND),
        Some(thread) => thread,
    };

    let comments = threads_repository
        .list_comments(thread_id)
        .await
        .or_internal_server_error()?;

    let response = ThreadResponse {
        id: thread_id,
        user_id: thread.user_id,
        title: thread.title,
        created_at: thread.created_at
            .and_utc()
            .to_rfc3339(),
        comments: comments
            .into_iter()
            .map(|comment| CommentResponse {
                id: comment.id,
                user_id: comment.user_id,
                content: comment.content,
                created_at: comment.created_at
                    .and_utc()
                    .to_rfc3339(),
            })
            .collect(),
    };

    Ok(Json(response))
}