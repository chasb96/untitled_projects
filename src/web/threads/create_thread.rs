use auth_client::axum::extractors::{Authenticate, ClaimsUser};
use axum::response::IntoResponse;
use axum::extract::Path;
use axum_extra::protobuf::Protobuf;
use or_status_code::OrInternalServerError;
use prost::Message;
use rand::distributions::{Alphanumeric, DistString};

use crate::web::ApiResult;
use crate::repository::threads::NewThread;
use crate::axum::extractors::threads_repository::ThreadsRepositoryExtractor;
use crate::repository::threads::ThreadsRepository;

#[derive(Message)]
pub struct CreateThreadRequest {
    #[prost(string, tag = "1")]
    pub title: String,
}

#[derive(Message)]
pub struct CreateThreadResponse {
    #[prost(string, tag = "1")]
    pub id: String,
}

pub async fn create_thread(
    Authenticate(user): Authenticate<ClaimsUser>,
    threads_repository: ThreadsRepositoryExtractor,
    Path(project_id): Path<String>,
    Protobuf(request): Protobuf<CreateThreadRequest>,
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

    Ok(Protobuf(CreateThreadResponse { 
        id: thread_id 
    }))
}