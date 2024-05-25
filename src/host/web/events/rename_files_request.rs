use std::collections::HashMap;

use rand::distributions::{Alphanumeric, DistString};
use serde::Deserialize;

use crate::host::events::RenameFilesEvent;

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