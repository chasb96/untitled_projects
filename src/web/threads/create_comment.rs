use auth_client::axum::extractors::{Authenticate, ClaimsUser};
use axum::{extract::Path, response::IntoResponse};
use axum::http::StatusCode;
use axum_extra::protobuf::Protobuf;
use or_status_code::OrInternalServerError;
use prost::Message;
use rand::distributions::{Alphanumeric, DistString};

use crate::{axum::extractors::threads_repository::ThreadsRepositoryExtractor, repository::threads::NewComment, web::{validate::{Validate, ValidationError}, ApiResult}};
use crate::repository::threads::ThreadsRepository;

#[derive(Message)]
pub struct CreateProjectRequest {
    #[prost(string, tag = "1")]
    pub content: String,
}

impl Validate for CreateProjectRequest {
    fn validate(&self) -> Result<(), ValidationError> {
        let content_len = self.content.len();

        if content_len == 0 { return Err("Content must have atleast one character".into()) }

        Ok(())
    }
}

#[derive(Message)]
pub struct CommentResponse {
    #[prost(string, tag = "1")]
    pub id: String,
}

pub async fn create_comment(
    Authenticate(user): Authenticate<ClaimsUser>,
    threads_repository: ThreadsRepositoryExtractor,
    Path((project_id, thread_id)): Path<(String, String)>,
    Protobuf(request): Protobuf<CreateProjectRequest>,
) ->ApiResult<impl IntoResponse> {
    let thread = threads_repository
        .get_by_id(&thread_id)
        .await
        .or_internal_server_error()?;

    let thread = match thread {
        None => return Err(StatusCode::NOT_FOUND),
        Some(thread) if thread.project_id != project_id => return Err(StatusCode::NOT_FOUND),
        Some(thread) => thread,
    };

    let comment_id = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);

    threads_repository
        .create_comment(NewComment {
            id: &comment_id,
            thread_id: &thread.id,
            user_id: &user.id,
            content: &request.content,
        })
        .await
        .or_internal_server_error()?;

    Ok(Protobuf(CommentResponse {
        id: comment_id
    }))
}