use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use super::source_request::FileMap;

#[derive(Serialize, Deserialize, Clone)]
pub struct CompletedSourceRequest {
    #[serde(rename = "p")]
    pub project_id: String,
    #[serde(rename = "u")]
    pub user_id: i32,
    #[serde(rename = "t")]
    pub title: String,
    #[serde(rename = "d")]
    pub description: String,
    #[serde(rename = "a")]
    pub approvers: HashSet<i32>,
    #[serde(rename = "f")]
    pub files: Vec<FileMap>,
}

#[derive(Deserialize)]
pub struct CompletedSourceRequestSummary {
    #[serde(rename = "p")]
    pub project_id: String,
    #[serde(rename = "u")]
    pub user_id: i32,
    #[serde(rename = "t")]
    pub title: String,
}