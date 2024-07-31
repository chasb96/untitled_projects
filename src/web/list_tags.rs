use axum::{extract::Path, response::IntoResponse};
use axum_extra::protobuf::Protobuf;
use or_status_code::OrInternalServerError;
use prost::Message;

use crate::axum::extractors::tags_repository::TagsRepositoryExtractor;
use crate::repository::tags::TagsRepository;

use super::ApiResult;

#[derive(Message)]
pub struct ListTagsResponse {
    #[prost(string, repeated, tag = "1")]
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

    Ok(Protobuf(ListTagsResponse { tags }))
}