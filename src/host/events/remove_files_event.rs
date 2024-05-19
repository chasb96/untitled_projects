use serde::{Deserialize, Serialize};

use super::{snapshot::Snapshot, Event, EventKind};

#[derive(Serialize, Deserialize, Clone)]
pub struct RemoveFilesEvent {
    pub event_id: String,
    pub paths: Vec<String>,
}

impl Event for RemoveFilesEvent {
    fn apply(self, entity: &mut Snapshot) {
        for path in self.paths {
            entity.files.remove(&path);
        }
        
        entity.event_id = self.event_id;
    }

    fn event_id(&self) -> &str {
        &self.event_id
    }
}

impl Into<EventKind> for RemoveFilesEvent {
    fn into(self) -> EventKind {
        EventKind::RemoveFile(self)
    }
}