use serde::{Deserialize, Serialize};

use super::{snapshot::Snapshot, Event, EventKind};

#[derive(Serialize, Deserialize, Clone)]
pub struct RemoveFilesEvent {
    #[serde(rename = "eid")]
    pub event_id: String,
    #[serde(rename = "p")]
    pub paths: Vec<String>,
    #[serde(rename = "pe")]
    pub previous_event_id: String,
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

    fn previous(&self) -> Option<&str> {
        Some(&self.previous_event_id)
    }
}

impl Into<EventKind> for RemoveFilesEvent {
    fn into(self) -> EventKind {
        EventKind::RemoveFiles(self)
    }
}