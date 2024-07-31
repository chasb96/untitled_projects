use axum::{extract::Path, response::IntoResponse};
use axum_extra::protobuf::Protobuf;
use prost::Message;
use axum::http::StatusCode;

use crate::axum::extractors::message_queue::MessageQueueExtractor;
use crate::message_queue::CreateTag;

use super::ApiResult;

#[derive(Message)]
pub struct CreateTagRequest {
    #[prost(string, tag = "1")]
    pub tag: String,
}

pub async fn create_tag(
    message_queue: MessageQueueExtractor,
    Path(project_id): Path<String>,
    Protobuf(request): Protobuf<CreateTagRequest>,
) -> ApiResult<impl IntoResponse> {
    message_queue
        .send(CreateTag {
            project_id,
            tag: request.tag.to_lowercase(),
        })
        .await;

    Ok(StatusCode::ACCEPTED)
}