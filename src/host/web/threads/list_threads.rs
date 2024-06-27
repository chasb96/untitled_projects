use axum::{extract::Path, response::IntoResponse, Json};
use or_status_code::OrInternalServerError;
use serde::Serialize;

use crate::host::{axum::extractors::threads_repository::ThreadsRepositoryExtractor, web::ApiResult};
use crate::host::repository::threads::ThreadsRepository;

#[derive(Serialize)]
pub struct ListThreadsResponse {
    #[serde(rename = "t")]
    threads: Vec<ThreadResponse>,
}

#[derive(Serialize)]
pub struct ThreadResponse {
    pub id: i32,
    #[serde(rename = "u")]
    pub user_id: i32,
    #[serde(rename = "t")]
    pub title: String,
    #[serde(rename = "cr")]
    pub created_at: String,
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
                created_at: thread.created_at
                    .and_utc()
                    .to_rfc3339(),
            })
            .collect(),
    };

    Ok(Json(response))
}