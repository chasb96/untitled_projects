use axum::{extract::Path, response::IntoResponse};
use axum::http::StatusCode;

use crate::axum::extractors::message_queue::MessageQueueExtractor;
use crate::message_queue::RemoveTag;

use super::ApiResult;

pub async fn remove_tag(
    message_queue: MessageQueueExtractor,
    Path((project_id, tag)): Path<(String, String)>,
) -> ApiResult<impl IntoResponse> {
    message_queue
        .send(RemoveTag {
            project_id,
            tag: tag.to_lowercase(),
        })
        .await;

    Ok(StatusCode::ACCEPTED)
}