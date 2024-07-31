use axum::{extract::Path, response::IntoResponse};
use axum_extra::protobuf::Protobuf;
use or_status_code::OrInternalServerError;
use prost::Message;

use crate::{axum::extractors::threads_repository::ThreadsRepositoryExtractor, web::ApiResult};
use crate::repository::threads::ThreadsRepository;

#[derive(Message)]
pub struct ListThreadsResponse {
    #[prost(message, repeated, tag = "1")]
    threads: Vec<ThreadResponse>,
}

#[derive(Message)]
pub struct ThreadResponse {
    #[prost(string, tag = "1")]
    pub id: String,
    #[prost(string, tag = "2")]
    pub user_id: String,
    #[prost(string, tag = "3")]
    pub title: String,
}

pub async fn list_threads(
    threads_repository: ThreadsRepositoryExtractor,
    Path(project_id): Path<String>,
) -> ApiResult<impl IntoResponse> {
    let threads = threads_repository
        .list(&project_id)
        .await
        .or_internal_server_error()?;

    let response = ListThreadsResponse {
        threads: threads
            .into_iter()
            .map(|thread| ThreadResponse {
                id: thread.id,
                user_id: thread.user_id,
                title: thread.title,
            })
            .collect(),
    };

    Ok(Protobuf(response))
}