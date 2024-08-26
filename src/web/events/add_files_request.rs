use rand::distributions::{Alphanumeric, DistString};
use serde::Deserialize;

use crate::{events::{AddFilesEvent, FileMap}, repository::EVENT_ID_LENGTH};

#[derive(Deserialize)]
pub struct AddFilesRequest {
    #[serde(rename = "pe")]
    pub previous_event_id: String,
    #[serde(rename = "f")]
    pub files: Vec<AddFileRequest>,
}

#[derive(Deserialize)]
pub struct AddFileRequest {
    #[serde(rename = "p")]
    pub path: String,
    #[serde(rename = "f")]
    pub file_id: String,
}

impl Into<AddFilesEvent> for AddFilesRequest {
    fn into(self) -> AddFilesEvent {
        AddFilesEvent {
            event_id: Alphanumeric.sample_string(&mut rand::thread_rng(), EVENT_ID_LENGTH),
            previous_event_id: self.previous_event_id,
            files: self.files
                .into_iter()
                .map(|file_request| FileMap {
                    path: file_request.path,
                    file_id: file_request.file_id,
                })
                .collect()
        }
    }
}