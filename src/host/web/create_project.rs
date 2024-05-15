use auth::client::axum::extractors::Authenticate;
use axum::body::Body;
use axum::http::Response;
use axum::response::IntoResponse;
use axum::{http::StatusCode, Json};
use or_status_code::OrInternalServerError;
use rand::distributions::{Alphanumeric, DistString};
use serde::{Deserialize, Serialize};
use users::client::{axum::extractors::UsersClient, ProjectRequest};

use crate::host::{axum::extractors::events_repository::EventsRepositoryExtractor, events::CreateEvent};
use crate::host::repository::events::EventsRepository;

use super::ApiResult;

#[derive(Deserialize)]
pub struct CreateProjectRequest {
    pub name: String,
}

#[derive(Serialize)]
pub struct CreateProjectResponse {
    pub id: String,
}

impl IntoResponse for CreateProjectResponse {
    fn into_response(self) -> Response<Body> {
        (StatusCode::CREATED, Json(self)).into_response()
    }
}

pub async fn create_project(
    Authenticate(user): Authenticate,
    events_repository: EventsRepositoryExtractor,
    UsersClient(client): UsersClient,
    Json(request): Json<CreateProjectRequest>
) -> ApiResult<Json<CreateProjectResponse>> {
    let project_id = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);

    let event = CreateEvent { 
        id: project_id.to_owned(), 
        event_id: Alphanumeric.sample_string(&mut rand::thread_rng(), 64),
        name: request.name, 
        owner_id: user.id 
    };

    events_repository
        .create(&project_id, event)
        .await
        .or_internal_server_error()?;

    let add_project_request = ProjectRequest {
        project_id: project_id.to_owned(),
    };

    client
        .add_project(user.id, add_project_request)
        .await
        .or_internal_server_error()?;

    Ok(Json(
        CreateProjectResponse {
            id: project_id,
        }
    ))
}