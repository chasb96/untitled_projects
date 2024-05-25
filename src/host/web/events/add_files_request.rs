use rand::distributions::{Alphanumeric, DistString};
use serde::Deserialize;

use crate::host::events::{AddFilesEvent, FileMap};

#[derive(Deserialize)]
pub struct AddFilesRequest {
    #[serde(rename = "f")]
    pub files: Vec<AddFileRequest>,
}

#[derive(Deserialize)]
pub struct AddFileRequest {
    #[serde(rename = "p")]
    pub path: String,
    #[serde(rename = "fid")]
    pub file_id: String,
}

impl Into<AddFilesEvent> for AddFilesRequest {
    fn into(self) -> AddFilesEvent {
        AddFilesEvent {
            event_id: Alphanumeric.sample_string(&mut rand::thread_rng(), 64),
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