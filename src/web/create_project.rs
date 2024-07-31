use axum::response::IntoResponse;
use axum::http::StatusCode;
use axum_extra::protobuf::Protobuf;
use or_status_code::OrInternalServerError;
use prost::Message;
use rand::distributions::{Alphanumeric, DistString};

use crate::axum::extractors::message_queue::MessageQueueExtractor;
use crate::message_queue::{CreateProject, CreateSnapshot};
use crate::repository::EVENT_ID_LENGTH;
use crate::{axum::extractors::events_repository::EventsRepositoryExtractor, events::CreateEvent};
use crate::repository::events::EventsRepository;

use super::ApiResult;

#[derive(Message)]
pub struct CreateProjectRequest {
    #[prost(string, tag = "1")]
    pub name: String,
    #[prost(string, tag = "2")]
    pub user_id: String,
}

#[derive(Message)]
pub struct CreateProjectResponse {
    #[prost(string, tag = "1")]
    pub id: String,
}

pub async fn create_project(
    events_repository: EventsRepositoryExtractor,
    message_queue: MessageQueueExtractor,
    Protobuf(request): Protobuf<CreateProjectRequest>
) -> ApiResult<impl IntoResponse> {
    let project_id = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);

    let event = CreateEvent { 
        id: project_id.to_owned(), 
        event_id: Alphanumeric.sample_string(&mut rand::thread_rng(), EVENT_ID_LENGTH),
        name: request.name.clone(), 
        owner_id: request.user_id.clone(),
    };

    events_repository
        .create(&project_id, event.clone())
        .await
        .or_internal_server_error()?;

    message_queue
        .send(CreateSnapshot {
            project_id: project_id.clone(),
            version: "latest".to_string(),
            snapshot: event.into(),
        })
        .await;

    message_queue
        .send(CreateProject {
            project_id: project_id.clone(),
            name: request.name,
        })
        .await;

    Ok((
        StatusCode::CREATED,
        Protobuf(CreateProjectResponse {
            id: project_id,
        }),
    ))
}