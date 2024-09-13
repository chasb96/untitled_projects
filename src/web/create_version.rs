use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_extra::protobuf::Protobuf;
use or_status_code::OrInternalServerError;
use or_status_code::OrNotFound;
use prost::Message;

use crate::axum::extractors::events_repository::EventsRepositoryExtractor;
use crate::axum::extractors::message_queue::MessageQueueExtractor;
use crate::events::Snapshot;
use crate::message_queue::CreateSnapshot;
use crate::repository::events::EventsRepository;

use super::ApiResult;

#[derive(Message)]
pub struct CreateVersionRequest {
    #[prost(string, tag = "1")]
    pub event_id: String,
    #[prost(string, tag = "2")]
    pub version: String,
}

pub async fn create_version(
    events_repository: EventsRepositoryExtractor,
    message_queue: MessageQueueExtractor,
    Path(project_id): Path<String>,
    Protobuf(request): Protobuf<CreateVersionRequest>
) -> ApiResult<impl IntoResponse> {
    let mut snapshot = Snapshot::new();

    events_repository
        .list(&project_id, &request.event_id)
        .await
        .or_internal_server_error()?
        .or_not_found()?
        .into_iter()
        .for_each(|event| snapshot.apply_event(event));

    message_queue
        .send(CreateSnapshot {
            project_id: project_id,
            version: request.version,
            snapshot,
        })
        .await;

    Ok(StatusCode::CREATED)
}