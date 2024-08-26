use std::collections::HashMap;

use rand::distributions::{Alphanumeric, DistString};
use serde::Deserialize;

use crate::{events::RenameFilesEvent, repository::EVENT_ID_LENGTH};

#[derive(Deserialize)]
pub struct RenameFilesRequest {
    #[serde(rename = "pe")]
    pub previous_event_id: String,
    #[serde(rename = "p")]
    pub paths: HashMap<String, String>,
}

impl Into<RenameFilesEvent> for RenameFilesRequest {
    fn into(self) -> RenameFilesEvent {
        RenameFilesEvent {
            event_id: Alphanumeric.sample_string(&mut rand::thread_rng(), EVENT_ID_LENGTH),
            previous_event_id: self.previous_event_id,
            paths: self.paths,
        }
    }
}