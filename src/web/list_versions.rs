use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_extra::protobuf::Protobuf;
use or_status_code::OrInternalServerError;
use prost::Message;

use crate::axum::extractors::snapshots_repository::SnapshotsRepositoryExtractor;
use crate::repository::snapshots::SnapshotsRepository;

use super::ApiResult;

#[derive(Message)]
pub struct ListVersionsResponse {
    #[prost(message, repeated, tag = "1")]
    pub versions: Vec<VersionResponse>,
}

#[derive(Message)]
pub struct VersionResponse {
    #[prost(string, tag = "1")]
    pub version: String,
    #[prost(string, tag = "2")]
    pub event_id: String,
}

pub async fn list_versions(
    snapshots_repository: SnapshotsRepositoryExtractor,
    Path(project_id): Path<String>
) -> ApiResult<impl IntoResponse> {
    let snapshots = snapshots_repository
        .list_versions(&project_id)
        .await
        .or_internal_server_error()?;

    if snapshots.len() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(Protobuf(ListVersionsResponse {
        versions: snapshots
            .into_iter()
            .map(|snapshot| VersionResponse {
                version: snapshot.version,
                event_id: snapshot.event_id,
            })
            .collect()
    }))
}