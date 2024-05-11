use axum::{extract::Path, http::{HeaderMap, StatusCode}, Json};
use rand::distributions::{Alphanumeric, DistString};

use crate::host::{axum::{extractors::{authenticate::AuthenticateExtractor, events_repository::EventsRepositoryExtractor}, JsonOrProtobuf}, events::{CreateEvent, EventKind}, util::or_status_code::{OrInternalServerError, OrStatusCode}};
use crate::host::repository::events::EventsRepository;

use super::{request::CreateProjectRequest, response::{CreateProjectResponse, ProjectResponse}};

pub async fn create_project(
    AuthenticateExtractor(user): AuthenticateExtractor,
    events_repository: EventsRepositoryExtractor,
    Json(request): Json<CreateProjectRequest>
) -> Result<(StatusCode, Json<CreateProjectResponse>), StatusCode> {
    let project_id = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
    let event_id = Alphanumeric.sample_string(&mut rand::thread_rng(), 64);

    let event = EventKind::Create(
        CreateEvent { 
            id: project_id.clone(), 
            event_id,
            name: request.name, 
            owner_id: user.id 
        }
    );
    
    events_repository
        .create(&project_id, event)
        .await
        .or_internal_server_error()?;

    Ok((
        StatusCode::CREATED,
        Json(
            CreateProjectResponse {
                id: project_id,
            }
        )
    ))
}

pub async fn get_project_by_id(
    events_repository: EventsRepositoryExtractor,
    headers: HeaderMap,
    Path(id): Path<String>
) -> Result<JsonOrProtobuf<ProjectResponse>, StatusCode> {
    let project = events_repository
        .get_by_id(&id)
        .await
        .or_internal_server_error()?
        .or_status_code(StatusCode::NOT_FOUND)?;

    let response_body = ProjectResponse {
        id: project.id,
        name: project.name,
        user_id: project.user_id,
    };

    Ok(JsonOrProtobuf::from((response_body, headers)))
}