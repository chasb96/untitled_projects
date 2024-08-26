use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use super::{completed::CompletedSourceRequest, source_request::FileMap};

#[derive(Serialize, Deserialize, Clone)]
pub struct ApprovedSourceRequest {
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
    #[serde(rename = "a")]
    pub approvers: HashSet<String>,
    #[serde(rename = "f")]
    pub files: Vec<FileMap>,
}

#[derive(Serialize, Deserialize)]
pub struct ApprovedSourceRequestSummary {
    #[serde(rename = "p")]
    pub project_id: String,
    #[serde(rename = "u")]
    pub user_id: String,
    #[serde(rename = "t")]
    pub title: String,
}

impl ApprovedSourceRequest {
    pub fn approve(self, approver: String) -> ApprovedSourceRequest {
        let mut approvers = self.approvers;

        approvers.insert(approver);

        ApprovedSourceRequest {
            project_id: self.project_id,
            user_id: self.user_id,
            title: self.title,
            description: self.description,
            previous_event_id: self.previous_event_id,
            approvers,
            files: self.files,
        }
    }

    pub fn complete(self) -> CompletedSourceRequest {
        CompletedSourceRequest {
            project_id: self.project_id,
            user_id: self.user_id,
            title: self.title,
            previous_event_id: self.previous_event_id,
            description: self.description,
            approvers: self.approvers,
            files: self.files,
        }
    }
}