use std::collections::HashMap;

use axum::extract::Path;
use axum::response::IntoResponse;
use axum::http::StatusCode;
use axum_extra::protobuf::Protobuf;
use or_status_code::{OrBadRequest, OrInternalServerError};
use prost::Message;

use crate::axum::extractors::{snapshots_repository::SnapshotsRepositoryExtractor, source_request_repository::SourceRequestsRepositoryExtractor};
use crate::repository::snapshots::SnapshotsRepository;
use crate::repository::source_requests::SourceRequestRepository;
use crate::web::ApiResult;

#[derive(Message)]
pub struct Diff {
    #[prost(map = "string, message", tag = "1")]
    pub diff_items: HashMap<String, DiffItem>
}

#[derive(PartialEq, Message)]
pub struct DiffItem {
    #[prost(string, tag = "1")]
    pub from: String,
    #[prost(string, tag = "2")]
    pub to: String,
}

pub async fn source_request_diff(
    snapshots_repository: SnapshotsRepositoryExtractor,
    source_request_repository: SourceRequestsRepositoryExtractor,
    Path((project_id, source_request_id)): Path<(String, String)>,
) -> ApiResult<impl IntoResponse> {
    let source_request = source_request_repository
        .get_by_id(&source_request_id)
        .await
        .or_internal_server_error()?
        .or_bad_request()?;

    if source_request.project_id() != project_id {
        return Err(StatusCode::NOT_FOUND);
    }

    let project = snapshots_repository
        .get_by_id(&project_id, "latest")
        .await
        .or_internal_server_error()?
        .or_bad_request()?;

    let response = Diff {
        diff_items: source_request
            .files()
            .into_iter()
            .filter(|file| project.files.contains_key(&file.path))
            .map(|file| (
                DiffItem {
                    from: project.files[&file.path].clone(), 
                    to: file.file_id
                },
                file.path
            ))
            .filter(|(diff_item, _)| diff_item.from != diff_item.to)
            .map(|(value, key)| (key, value))
            .collect()
    };

    Ok(Protobuf(response))
}