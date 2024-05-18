use serde::{Deserialize, Serialize};

use super::{snapshot::Snapshot, Event, EventKind};

#[derive(Serialize, Deserialize)]
pub struct RemoveFilesEvent {
    pub event_id: String,
    pub path: String,
}

impl Event for RemoveFilesEvent {
    fn apply(self, entity: &mut Snapshot) {
        entity.files.remove(&self.path);
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