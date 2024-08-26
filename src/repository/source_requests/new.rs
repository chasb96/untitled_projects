use serde::{Deserialize, Serialize};

use super::{approved::ApprovedSourceRequest, source_request::{FileMap, NewFileMap}};

#[derive(Serialize, Deserialize)]
pub struct NewSourceRequest {
    #[serde(rename = "p")]
    pub project_id: String,
    #[serde(rename = "u")]
    pub user_id: String,
    #[serde(rename = "t")]
    pub title: String,
    #[serde(rename = "d")]
    pub description: String,
    #[serde(rename = "pe")]
    pub previous_event_id: String,
    #[serde(rename = "f")]
    pub files: Vec<FileMap>,
}

#[derive(Deserialize)]
pub struct NewSourceRequestSummary {
    #[serde(rename = "p")]
    pub project_id: String,
    #[serde(rename = "u")]
    pub user_id: String,
    #[serde(rename = "t")]
    pub title: String,
}

#[derive(Serialize)]
pub struct CreateNewSourceRequest<'a> {
    #[serde(rename = "p")]
    pub project_id: &'a str,
    #[serde(rename = "u")]
    pub user_id: &'a str,
    #[serde(rename = "t")]
    pub title: &'a str,
    #[serde(rename = "d")]
    pub description: &'a str,
    #[serde(rename = "pe")]
    pub previous_event_id: &'a str,
    #[serde(rename = "f")]
    pub files: Vec<NewFileMap<'a>>,
}

impl NewSourceRequest {
    pub fn approve(self, approver: String) -> ApprovedSourceRequest {
        ApprovedSourceRequest {
            project_id: self.project_id,
            user_id: self.user_id,
            title: self.title,
            description: self.description,
            previous_event_id: self.previous_event_id,
            approvers: vec![approver]
                .into_iter()
                .collect(),
            files: self.files,
        }
    }
}