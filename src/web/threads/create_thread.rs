use auth_client::axum::extractors::{Authenticate, ClaimsUser};
use axum::{extract::Path, response::IntoResponse, Json};
use or_status_code::OrInternalServerError;
use rand::distributions::{Alphanumeric, DistString};
use serde::{Deserialize, Serialize};

use crate::{axum::extractors::{threads_repository::ThreadsRepositoryExtractor, validate::Validated}, repository::threads::NewThread, web::{validate::{Validate, ValidationError}, ApiResult}};
use crate::repository::threads::ThreadsRepository;

#[derive(Deserialize)]
pub struct CreateThreadRequest {
    #[serde(rename = "t")]
    pub title: String,
}

impl Validate for CreateThreadRequest {
    fn validate(&self) -> Result<(), ValidationError> {
        let title_len = self.title.len();

        if title_len == 0 { return Err("Title must have atleast one character".into()) }
        if title_len > 128 { return Err("Title cannot be more than 32 characters".into()) }

        Ok(())
    }
}

#[derive(Serialize)]
pub struct CreateThreadResponse {
    pub id: String,
}

pub async fn create_thread(
    Authenticate(user): Authenticate<ClaimsUser>,
    threads_repository: ThreadsRepositoryExtractor,
    Path(project_id): Path<String>,
    Validated(Json(request)): Validated<Json<CreateThreadRequest>>,
) -> ApiResult<impl IntoResponse> {
    let thread_id = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);

    threads_repository
        .create(NewThread {
            id: &thread_id,
            project_id: &project_id,
            user_id: &user.id,
            title: &request.title,
        })
        .await
        .or_internal_server_error()?;

    Ok(Json(CreateThreadResponse { 
        id: thread_id 
    }))
}