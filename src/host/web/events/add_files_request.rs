use rand::distributions::{Alphanumeric, DistString};
use serde::Deserialize;

use crate::host::{events::{AddFilesEvent, FileMap}, repository::EVENT_ID_LENGTH, web::validate::{Validate, ValidationError}};

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
            event_id: Alphanumeric.sample_string(&mut rand::thread_rng(), EVENT_ID_LENGTH),
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

impl Validate for AddFilesRequest {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.files.is_empty() { return Err("Must add atleast one file".into()); }

        for file in &self.files {
            file.validate()?;
        }

        Ok(())
    }
}

impl Validate for AddFileRequest {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.path.starts_with(' ') { return Err("Path cannot start with whitespace".into()); }
        if self.path.ends_with(' ') { return Err("Path cannot end with whitespace".into()); }
        if self.path.is_empty() { return Err("Path cannot be empty".into()); }
        if self.file_id.is_empty() { return Err("File ID cannot be empty".into()); }

        Ok(())
    }
}