mod create_souce_request;
mod list_source_requests_by_project;
mod get_source_request;
mod approve_source_request;
mod complete_source_request;
pub mod comments;

use std::collections::HashSet;

pub use create_souce_request::create_source_request;
pub use list_source_requests_by_project::list_source_requests_by_project;
pub use get_source_request::get_source_request;
pub use approve_source_request::approve_source_request;
pub use complete_source_request::complete_source_request;

use serde::{Deserialize, Serialize};

use crate::host::repository::source_requests;

#[derive(Serialize)]
struct ListSourceRequests {
    #[serde(rename = "sr")]
    source_requests: Vec<ListSourceRequestItem>,
}

#[derive(Serialize)]
struct ListSourceRequestItem {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "sr")]
    pub source_request: SourceRequestSummary,
}

#[derive(Serialize, Deserialize)]
enum SourceRequest {
    #[serde(rename = "n")]
    New(NewSourceRequest),
    #[serde(rename = "a")]
    Approved(ApprovedSourceRequest),
    #[serde(rename = "c")]
    Completed(CompletedSourceRequest),
}

#[derive(Serialize, Deserialize)]
enum SourceRequestSummary {
    #[serde(rename = "n")]
    New(NewSourceRequestSummary),
    #[serde(rename = "a")]
    Approved(ApprovedSourceRequestSummary),
    #[serde(rename = "c")]
    Completed(CompletedSourceRequestSummary),
}

impl From<source_requests::SourceRequest> for SourceRequest {
    fn from(source_request: source_requests::SourceRequest) -> Self {
        match source_request {
            source_requests::SourceRequest::New(new) => SourceRequest::New(NewSourceRequest::from(new)),
            source_requests::SourceRequest::Approved(approved) => SourceRequest::Approved(ApprovedSourceRequest::from(approved)),
            source_requests::SourceRequest::Completed(completed) => SourceRequest::Completed(CompletedSourceRequest::from(completed)),
        }
    }
}

impl From<source_requests::SourceRequestSummary> for SourceRequestSummary {
    fn from(source_request_summary: source_requests::SourceRequestSummary) -> Self {
        match source_request_summary {
            source_requests::SourceRequestSummary::New(new) => SourceRequestSummary::New(NewSourceRequestSummary::from(new)),
            source_requests::SourceRequestSummary::Approved(approved) => SourceRequestSummary::Approved(ApprovedSourceRequestSummary::from(approved)),
            source_requests::SourceRequestSummary::Completed(completed) => SourceRequestSummary::Completed(CompletedSourceRequestSummary::from(completed)),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct FileMap {
    #[serde(rename = "p")]
    pub path: String,
    #[serde(rename = "f")]
    pub file_id: String,
}

impl From<source_requests::FileMap> for FileMap {
    fn from(file_map: source_requests::FileMap) -> Self {
        FileMap {
            path: file_map.path,
            file_id: file_map.file_id,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct NewSourceRequest {
    #[serde(rename = "p")]
    project_id: String,
    #[serde(rename = "u")]
    user_id: i32,
    #[serde(rename = "t")]
    title: String,
    #[serde(rename = "d")]
    description: String,
    #[serde(rename = "f")]
    files: Vec<FileMap>,
}

#[derive(Serialize, Deserialize)]
struct NewSourceRequestSummary {
    #[serde(rename = "p")]
    project_id: String,
    #[serde(rename = "u")]
    user_id: i32,
    #[serde(rename = "t")]
    title: String,
}

impl From<source_requests::NewSourceRequest> for NewSourceRequest {
    fn from(new_source_request: source_requests::NewSourceRequest) -> Self {
        NewSourceRequest {
            project_id: new_source_request.project_id,
            user_id: new_source_request.user_id,
            title: new_source_request.title,
            description: new_source_request.description,
            files: new_source_request.files
                .into_iter()
                .map(FileMap::from)
                .collect(),
        }
    }
}

impl From<source_requests::NewSourceRequestSummary> for NewSourceRequestSummary {
    fn from(new_source_request_summary: source_requests::NewSourceRequestSummary) -> Self {
        NewSourceRequestSummary {
            project_id: new_source_request_summary.project_id,
            user_id: new_source_request_summary.user_id,
            title: new_source_request_summary.title,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct ApprovedSourceRequest {
    #[serde(rename = "p")]
    project_id: String,
    #[serde(rename = "u")]
    user_id: i32,
    #[serde(rename = "t")]
    title: String,
    #[serde(rename = "d")]
    description: String,
    #[serde(rename = "a")]
    approvers: HashSet<i32>,
    #[serde(rename = "f")]
    files: Vec<FileMap>,
}

#[derive(Serialize, Deserialize)]
struct ApprovedSourceRequestSummary {
    #[serde(rename = "p")]
    project_id: String,
    #[serde(rename = "u")]
    user_id: i32,
    #[serde(rename = "t")]
    title: String,
}

impl From<source_requests::ApprovedSourceRequest> for ApprovedSourceRequest {
    fn from(approved_source_request: source_requests::ApprovedSourceRequest) -> Self {
        ApprovedSourceRequest {
            project_id: approved_source_request.project_id,
            user_id: approved_source_request.user_id,
            title: approved_source_request.title,
            description: approved_source_request.description,
            approvers: approved_source_request.approvers,
            files: approved_source_request.files
                .into_iter()
                .map(FileMap::from)
                .collect(),
        }
    }
}

impl From<source_requests::ApprovedSourceRequestSummary> for ApprovedSourceRequestSummary {
    fn from(approved_source_request_summary: source_requests::ApprovedSourceRequestSummary) -> Self {
        ApprovedSourceRequestSummary {
            project_id: approved_source_request_summary.project_id,
            user_id: approved_source_request_summary.user_id,
            title: approved_source_request_summary.title,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct CompletedSourceRequest {
    #[serde(rename = "p")]
    project_id: String,
    #[serde(rename = "u")]
    user_id: i32,
    #[serde(rename = "t")]
    title: String,
    #[serde(rename = "d")]
    description: String,
    #[serde(rename = "a")]
    approvers: HashSet<i32>,
    #[serde(rename = "f")]
    files: Vec<FileMap>,
}

#[derive(Serialize, Deserialize)]
struct CompletedSourceRequestSummary {
    #[serde(rename = "p")]
    project_id: String,
    #[serde(rename = "u")]
    user_id: i32,
    #[serde(rename = "t")]
    title: String,
}

impl From<source_requests::CompletedSourceRequest> for CompletedSourceRequest {
    fn from(completed_source_request: source_requests::CompletedSourceRequest) -> Self {
        CompletedSourceRequest {
            project_id: completed_source_request.project_id,
            user_id: completed_source_request.user_id,
            title: completed_source_request.title,
            description: completed_source_request.description,
            approvers: completed_source_request.approvers,
            files: completed_source_request.files
                .into_iter()
                .map(FileMap::from)
                .collect(),
        }
    }
}

impl From<source_requests::CompletedSourceRequestSummary> for CompletedSourceRequestSummary {
    fn from(completed_source_request_summary: source_requests::CompletedSourceRequestSummary) -> Self {
        CompletedSourceRequestSummary {
            project_id: completed_source_request_summary.project_id,
            user_id: completed_source_request_summary.user_id,
            title: completed_source_request_summary.title,
        }
    }
}