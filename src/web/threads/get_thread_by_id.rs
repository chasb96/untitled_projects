use axum::{extract::Path, response::IntoResponse};
use axum_extra::protobuf::Protobuf;
use or_status_code::OrInternalServerError;
use prost::Message;
use axum::http::StatusCode;

use crate::{axum::extractors::threads_repository::ThreadsRepositoryExtractor, web::ApiResult};
use crate::repository::threads::ThreadsRepository;

#[derive(Message)]
pub struct ThreadResponse {
    #[prost(string, tag = "1")]
    pub id: String,
    #[prost(string, tag = "2")]
    pub user_id: String,
    #[prost(string, tag = "3")]
    pub title: String,
    #[prost(message, repeated, tag = "4")]
    pub comments: Vec<CommentResponse>,
}

#[derive(Message)]
pub struct CommentResponse {
    #[prost(string, tag = "1")]
    pub id: String,
    #[prost(string, tag = "2")]
    pub user_id: String,
    #[prost(string, tag = "3")]
    pub content: String,
}

pub async fn get_thread_by_id(
    threads_repository: ThreadsRepositoryExtractor,
    Path((project_id, thread_id)): Path<(String, String)>,
) -> ApiResult<impl IntoResponse> {
    let thread = threads_repository
        .get_by_id(&thread_id)
        .await
        .or_internal_server_error()?;

    let thread = match thread {
        None => return Err(StatusCode::NOT_FOUND),
        Some(thread) if thread.project_id != project_id => return Err(StatusCode::NOT_FOUND),
        Some(thread) => thread,
    };

    let comments = threads_repository
        .list_comments(&thread_id)
        .await
        .or_internal_server_error()?;

    let response = ThreadResponse {
        id: thread_id,
        user_id: thread.user_id,
        title: thread.title,
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