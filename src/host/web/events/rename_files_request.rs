use std::collections::HashMap;

use rand::distributions::{Alphanumeric, DistString};
use serde::Deserialize;

use crate::host::{events::RenameFilesEvent, web::validate::{Validate, ValidationError}};

#[derive(Deserialize)]
pub struct RenameFilesRequest {
    #[serde(rename = "p")]
    pub paths: HashMap<String, String>,
}

impl Into<RenameFilesEvent> for RenameFilesRequest {
    fn into(self) -> RenameFilesEvent {
        RenameFilesEvent {
            event_id: Alphanumeric.sample_string(&mut rand::thread_rng(), 64),
            paths: self.paths,
        }
    }
}

impl Validate for RenameFilesRequest {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.paths.is_empty() { return Err("Must rename atleast one file".into()); }

        for (old_path, new_path) in &self.paths {
            if old_path.starts_with(' ') { return Err("Old path cannot start with whitespace".into()); }
            if old_path.ends_with(' ') { return Err("Old path cannot end with whitespace".into()); }
            if old_path.is_empty() { return Err("Old path cannot be empty".into()); }
            if new_path.starts_with(' ') { return Err("New path cannot start with whitespace".into()); }
            if new_path.ends_with(' ') { return Err("New path cannot end with whitespace".into()); }
            if new_path.is_empty() { return Err("New path cannot be empty".into()); }
        }

        Ok(())
    }
}