use rand::distributions::{Alphanumeric, DistString};
use serde::Deserialize;

use crate::host::{events::RemoveFilesEvent, repository::EVENT_ID_LENGTH, web::validate::{Validate, ValidationError}};

#[derive(Deserialize)]
pub struct RemoveFilesRequest {
    #[serde(rename = "p")]
    pub paths: Vec<String>,
}

impl Into<RemoveFilesEvent> for RemoveFilesRequest {
    fn into(self) -> RemoveFilesEvent {
        RemoveFilesEvent {
            event_id: Alphanumeric.sample_string(&mut rand::thread_rng(), EVENT_ID_LENGTH),
            paths: self.paths,
        }
    }
}

impl Validate for RemoveFilesRequest {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.paths.is_empty() { return Err("Must remove atleast one file".into()); }

        for path in &self.paths {
            if path.starts_with(' ') { return Err("Path cannot start with whitespace".into()); }
            if path.ends_with(' ') { return Err("Path cannot end with whitespace".into()); }
            if path.is_empty() { return Err("Path cannot be empty".into()); }
        }

        Ok(())
    }
}