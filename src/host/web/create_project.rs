use auth::client::axum::extractors::{Authenticate, ClaimsUser};
use axum::response::IntoResponse;
use axum::{http::StatusCode, Json};
use or_status_code::OrInternalServerError;
use rand::distributions::{Alphanumeric, DistString};
use serde::{Deserialize, Serialize};

use crate::host::axum::extractors::message_queue::MessageQueueExtractor;
use crate::host::axum::extractors::validate::Validated;
use crate::host::message_queue::{AssignProject, CreateProject, CreateSnapshot};
use crate::host::repository::EVENT_ID_LENGTH;
use crate::host::{axum::extractors::events_repository::EventsRepositoryExtractor, events::CreateEvent};
use crate::host::repository::events::EventsRepository;

use super::validate::{Validate, ValidationError};
use super::ApiResult;

#[derive(Deserialize)]
pub struct CreateProjectRequest {
    pub name: String,
}

impl Validate for CreateProjectRequest {
    fn validate(&self) -> Result<(), ValidationError> {
        let name_len = self.name.len();

        if name_len == 0 { return Err("Name must have atleast one character".into()) }
        if name_len > 32 { return Err("Name cannot be more than 32 characters".into()) }

        Ok(())
    }
}

#[derive(Serialize)]
pub struct CreateProjectResponse {
    pub id: String,
}

pub async fn create_project(
    Authenticate(user): Authenticate<ClaimsUser>,
    events_repository: EventsRepositoryExtractor,
    message_queue: MessageQueueExtractor,
    Validated(Json(request)): Validated<Json<CreateProjectRequest>>
) -> ApiResult<impl IntoResponse> {
    let project_id = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);

    let event = CreateEvent { 
        id: project_id.to_owned(), 
        event_id: Alphanumeric.sample_string(&mut rand::thread_rng(), EVENT_ID_LENGTH),
        name: request.name.clone(), 
        owner_id: user.id.clone(),
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
        .send(AssignProject {
            user_id: user.id,
            project_id: project_id.clone(),
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
        Json(CreateProjectResponse {
            id: project_id,
        }),
    ))
}