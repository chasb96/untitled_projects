use rand::distributions::{Alphanumeric, DistString};
use serde::Deserialize;

use crate::host::events::RemoveFilesEvent;

#[derive(Deserialize)]
pub struct RemoveFilesRequest {
    #[serde(rename = "p")]
    pub paths: Vec<String>,
}

impl Into<RemoveFilesEvent> for RemoveFilesRequest {
    fn into(self) -> RemoveFilesEvent {
        RemoveFilesEvent {
            event_id: Alphanumeric.sample_string(&mut rand::thread_rng(), 64),
            paths: self.paths,
        }
    }
}