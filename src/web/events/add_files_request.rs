use prost::Message;
use rand::distributions::{Alphanumeric, DistString};

use crate::{events::{AddFilesEvent, FileMap}, repository::EVENT_ID_LENGTH, web::validate::{Validate, ValidationError}};

#[derive(Message)]
pub struct AddFilesRequest {
    #[prost(message, repeated, tag = "1")]
    pub files: Vec<AddFileRequest>,
}

#[derive(Message)]
pub struct AddFileRequest {
    #[prost(string, tag = "1")]
    pub path: String,
    #[prost(string, tag = "2")]
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