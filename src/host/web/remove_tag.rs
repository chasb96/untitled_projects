use auth::client::axum::extractors::{Authenticate, ClaimsUser};
use axum::{extract::Path, response::IntoResponse};
use or_status_code::{OrInternalServerError, OrNotFound};
use axum::http::StatusCode;

use crate::host::axum::extractors::{snapshots_repository::SnapshotsRepositoryExtractor, tags_repository::TagsRepositoryExtractor};
use crate::host::repository::snapshots::SnapshotsRepository;
use crate::host::repository::tags::TagsRepository;

use super::ApiResult;

pub async fn remove_tag(
    // Authenticate(user): Authenticate<ClaimsUser>,
    snapshots_repository: SnapshotsRepositoryExtractor,
    tags_repository: TagsRepositoryExtractor,
    Path((project_id, tag)): Path<(String, String)>,
) -> ApiResult<impl IntoResponse> {
    let project = snapshots_repository
        .get_by_id(&project_id, "latest")
        .await
        .or_internal_server_error()?
        .or_not_found()?;

    // if project.user_id != user.id {
    //     return Err(StatusCode::FORBIDDEN);
    // }

    tags_repository
        .delete(&project_id, &tag.to_lowercase())
        .await
        .or_internal_server_error()?;

    Ok(StatusCode::NO_CONTENT)
}