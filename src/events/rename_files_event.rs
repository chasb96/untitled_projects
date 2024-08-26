use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{Event, EventKind, Snapshot};

#[derive(Serialize, Deserialize, Clone)]
pub struct RenameFilesEvent {
    #[serde(rename = "eid")]
    pub event_id: String,
    #[serde(rename = "p")]
    pub paths: HashMap<String, String>,
    #[serde(rename = "pe")]
    pub previous_event_id: String,
}

impl Event for RenameFilesEvent {
    fn apply(self, entity: &mut Snapshot) {
        for (old_path, new_path) in self.paths {
            if let Some(file_id) = entity.files.remove(&old_path) {
                entity.files.insert(new_path, file_id);
            }
        }
        
        entity.event_id = self.event_id;
    }

    fn event_id(&self) -> &str {
        &self.event_id
    }

    fn previous(&self) -> Option<&str> {
        Some(&self.previous_event_id)
    }
}

impl Into<EventKind> for RenameFilesEvent {
    fn into(self) -> EventKind {
        EventKind::RenameFiles(self)
    }
}