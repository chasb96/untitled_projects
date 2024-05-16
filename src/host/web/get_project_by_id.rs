use std::collections::HashMap;

use axum::{extract::{Path, Query}, http::{HeaderMap, StatusCode}, response::IntoResponse};
use json_or_protobuf::JsonOrProtobuf;
use or_status_code::{OrInternalServerError, OrStatusCode};
use serde::Deserialize;

use crate::host::axum::extractors::snapshots_repository::SnapshotsRepositoryExtractor;
use crate::host::repository::snapshots::SnapshotsRepository;

use super::ApiResult;

use prost::Message;
use serde::Serialize;

#[derive(Serialize, Message)]
pub struct ProjectResponse {
    #[prost(string, tag = "1")]
    pub id: String,
    #[prost(string, tag = "2")]
    pub name: String,
    #[prost(int32, tag = "3")]
    pub user_id: i32,
    #[prost(map= "string, string", tag = "4")]
    pub files: HashMap<String, String>,
}

#[derive(Deserialize)]
pub struct GetProjectByIdQuery {
    pub version: Option<String>
}

pub async fn get_project_by_id(
    snapshots_repository: SnapshotsRepositoryExtractor,
    headers: HeaderMap,
    Path(id): Path<String>,
    query: Query<GetProjectByIdQuery>
) -> ApiResult<impl IntoResponse> {
    let version = match &query.version {
        Some(version) => version,
        None => "latest"
    };

    let project = snapshots_repository
        .get_by_id(&id, version)
        .await
        .or_internal_server_error()?
        .or_status_code(StatusCode::NOT_FOUND)?;

    let response_body = ProjectResponse {
        id: project.id,
        name: project.name,
        user_id: project.user_id,
        files: project.files,
    };

    Ok(JsonOrProtobuf::from_accept_header(response_body, &headers))
}