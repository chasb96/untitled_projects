use serde::{Deserialize, Serialize};

use super::{approved::{ApprovedSourceRequest, ApprovedSourceRequestSummary}, new::{NewSourceRequest, NewSourceRequestSummary}, CompletedSourceRequest, CompletedSourceRequestSummary, CreateNewSourceRequest};

#[derive(Serialize)]
pub enum CreateSourceRequest<'a> {
    New(CreateNewSourceRequest<'a>),
}

#[derive(Serialize, Deserialize)]
pub enum SourceRequest {
    New(NewSourceRequest),
    Approved(ApprovedSourceRequest),
    Completed(CompletedSourceRequest),
}

#[derive(Deserialize)]
pub enum SourceRequestSummary {
    New(NewSourceRequestSummary),
    Approved(ApprovedSourceRequestSummary),
    Completed(CompletedSourceRequestSummary),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FileMap {
    #[serde(rename = "p")]
    pub path: String,
    #[serde(rename = "f")]
    pub file_id: String,
}

#[derive(Serialize)]
pub struct NewFileMap<'a> {
    #[serde(rename = "p")]
    pub path: &'a str,
    #[serde(rename = "f")]
    pub file_id: &'a str,
}

impl<'a> CreateSourceRequest<'a> {
    pub fn project_id(&self) -> &str {
        match self {
            CreateSourceRequest::New(new) => new.project_id,
        }
    }

    pub fn user_id(&self) -> i32 {
        match self {
            CreateSourceRequest::New(new) => new.user_id,
        }
    }

    pub fn state_i16(&self) -> i16 {
        match self {
            CreateSourceRequest::New(_) => 0,
        }
    }
}

impl SourceRequest {
    pub fn project_id(&self) -> &str {
        match self {
            SourceRequest::New(new) => &new.project_id,
            SourceRequest::Approved(approved) => &approved.project_id,
            SourceRequest::Completed(completed) => &completed.project_id,
        }
    }

    pub fn user_id(&self) -> i32 {
        match self {
            SourceRequest::New(new) => new.user_id,
            SourceRequest::Approved(approved) => approved.user_id,
            SourceRequest::Completed(completed) => completed.user_id,
        }
    }

    pub fn state_i16(&self) -> i16 {
        match self {
            SourceRequest::New(_) => 0,
            SourceRequest::Approved(_) => 1,
            SourceRequest::Completed(_) => 2,
        }
    }
}

impl<'a> From<CreateNewSourceRequest<'a>> for CreateSourceRequest<'a> {
    fn from(new: CreateNewSourceRequest<'a>) -> Self {
        CreateSourceRequest::New(new)
    }
}

impl From<NewSourceRequest> for SourceRequest {
    fn from(new: NewSourceRequest) -> Self {
        SourceRequest::New(new)
    }
}

impl From<ApprovedSourceRequest> for SourceRequest {
    fn from(approved: ApprovedSourceRequest) -> Self {
        SourceRequest::Approved(approved)
    }
}

impl From<CompletedSourceRequest> for SourceRequest {
    fn from(completed: CompletedSourceRequest) -> Self {
        SourceRequest::Completed(completed)
    }
}