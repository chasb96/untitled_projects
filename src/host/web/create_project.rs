use auth::client::axum::extractors::{Authenticate, ClaimsUser};
use axum::response::IntoResponse;
use axum::{http::StatusCode, Json};
use or_status_code::OrInternalServerError;
use rand::distributions::{Alphanumeric, DistString};
use serde::{Deserialize, Serialize};
use users::client::{axum::extractors::UsersClient, ProjectRequest};

use crate::host::axum::extractors::snapshots_repository::SnapshotsRepositoryExtractor;
use crate::host::{axum::extractors::events_repository::EventsRepositoryExtractor, events::CreateEvent};
use crate::host::repository::events::EventsRepository;
use crate::host::repository::snapshots::SnapshotsRepository;

use super::ApiResult;

#[derive(Deserialize)]
pub struct CreateProjectRequest {
    pub name: String,
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
    Json(request): Json<CreateProjectRequest>
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