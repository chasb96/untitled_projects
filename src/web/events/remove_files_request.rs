use rand::distributions::{Alphanumeric, DistString};
use serde::Deserialize;

use crate::{events::RemoveFilesEvent, repository::EVENT_ID_LENGTH};

#[derive(Deserialize)]
pub struct RemoveFilesRequest {
    #[serde(rename = "pe")]
    pub previous_event_id: String,
    #[serde(rename = "p")]
    pub paths: Vec<String>,
}

impl Into<RemoveFilesEvent> for RemoveFilesRequest {
    fn into(self) -> RemoveFilesEvent {
        RemoveFilesEvent {
            event_id: Alphanumeric.sample_string(&mut rand::thread_rng(), EVENT_ID_LENGTH),
            previous_event_id: self.previous_event_id,
            paths: self.paths,
        }
    }
}