use auth::client::axum::extractors::{Authenticate, ClaimsUser};
use axum::response::IntoResponse;
use axum::{http::StatusCode, Json};
use or_status_code::OrInternalServerError;
use rand::distributions::{Alphanumeric, DistString};
use serde::{Deserialize, Serialize};
use users::client::{axum::extractors::UsersClient, ProjectRequest};

use crate::host::axum::extractors::snapshots_repository::SnapshotsRepositoryExtractor;
use crate::host::axum::extractors::validate::Validated;
use crate::host::{axum::extractors::events_repository::EventsRepositoryExtractor, events::CreateEvent};
use crate::host::repository::events::EventsRepository;
use crate::host::repository::snapshots::SnapshotsRepository;

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
    snapshots_repository: SnapshotsRepositoryExtractor,
    UsersClient(client): UsersClient,
    Validated(Json(request)): Validated<Json<CreateProjectRequest>>
) -> ApiResult<impl IntoResponse> {
    let project_id = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);

    let event = CreateEvent { 
        id: project_id.to_owned(), 
        event_id: Alphanumeric.sample_string(&mut rand::thread_rng(), 64),
        name: request.name, 
        owner_id: user.id 
    };

    events_repository
        .create(&project_id, event.clone())
        .await
        .or_internal_server_error()?;

    snapshots_repository
        .create(&project_id, "latest", event)
        .await
        .or_internal_server_error()?;

    let add_project_request = ProjectRequest {
        project_id: project_id.to_owned(),
    };

    client
        .add_project(user.id, add_project_request)
        .await
        .or_internal_server_error()?;

    let response = (
        StatusCode::CREATED,
        Json(CreateProjectResponse {
            id: project_id,
        }),
    );

    Ok(response)
}