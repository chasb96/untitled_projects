use axum::Json;
use axum::{extract::Path, response::IntoResponse};
use or_status_code::OrInternalServerError;
use serde::Serialize;

use crate::host::axum::extractors::tags_repository::TagsRepositoryExtractor;
use crate::host::repository::tags::TagsRepository;

use super::ApiResult;

#[derive(Serialize)]
pub struct ListTagsResponse {
    #[serde(rename = "t")]
    tags: Vec<String>,
}

pub async fn list_tags(
    tags_repository: TagsRepositoryExtractor,
    Path(project_id): Path<String>,
) -> ApiResult<impl IntoResponse> {
    let tags = tags_repository
        .list(&project_id)
        .await
        .or_internal_server_error()?;

    Ok(Json(ListTagsResponse { tags }))
}