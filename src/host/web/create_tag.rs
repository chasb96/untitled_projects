use auth::client::axum::extractors::{Authenticate, ClaimsUser};
use axum::{extract::Path, response::IntoResponse, Json};
use or_status_code::{OrInternalServerError, OrNotFound};
use serde::Deserialize;
use axum::http::StatusCode;

use crate::host::axum::extractors::validate::Validated;
use crate::host::axum::extractors::{snapshots_repository::SnapshotsRepositoryExtractor, tags_repository::TagsRepositoryExtractor};
use crate::host::repository::snapshots::SnapshotsRepository;
use crate::host::repository::tags::TagsRepository;

use super::validate::{Validate, ValidationError};
use super::ApiResult;

#[derive(Deserialize)]
pub struct CreateTagRequest {
    #[serde(rename = "t")]
    pub tag: String,
}

impl Validate for CreateTagRequest {
    fn validate(&self) -> Result<(), ValidationError> {
        let tag_len = self.tag.len();

        if self.tag.contains(' ') { return Err("Tag cannot contain whitespace".into()) }
        if tag_len == 0 { return Err("Tag must have atleast one character".into()) }
        if tag_len > 16 { return Err("Tag cannot be more than 16 characters".into()) }

        Ok(())
    }
}

pub async fn create_tag(
    Authenticate(user): Authenticate<ClaimsUser>,
    snapshots_repository: SnapshotsRepositoryExtractor,
    tags_repository: TagsRepositoryExtractor,
    Path(project_id): Path<String>,
    Validated(Json(request)): Validated<Json<CreateTagRequest>>,
) -> ApiResult<impl IntoResponse> {
    let project = snapshots_repository
        .get_by_id(&project_id, "latest")
        .await
        .or_internal_server_error()?
        .or_not_found()?;

    if project.user_id != user.id {
        return Err(StatusCode::FORBIDDEN);
    }

    let tags = tags_repository
        .list(&project_id)
        .await
        .or_internal_server_error()?;

    if !tags.contains(&request.tag.to_lowercase()) {
        tags_repository
            .create(&project_id, &request.tag.to_lowercase())
            .await
            .or_internal_server_error()?;
    }

    Ok(StatusCode::ACCEPTED)
}