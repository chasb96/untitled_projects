use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{Event, EventKind, Snapshot};

#[derive(Serialize, Deserialize, Clone)]
pub struct RenameFilesEvent {
    pub event_id: String,
    pub paths: HashMap<String, String>,
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
}

impl Into<EventKind> for RenameFilesEvent {
    fn into(self) -> EventKind {
        EventKind::RenameFiles(self)
    }
}