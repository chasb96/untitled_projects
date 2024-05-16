use serde::{Deserialize, Serialize};

use super::{snapshot::Snapshot, Event, EventKind};

#[derive(Serialize, Deserialize)]
pub struct AddFileEvent {
    pub event_id: String,
    pub path: String,
    pub file_id: String,
}

impl Event for AddFileEvent {
    fn apply(self, entity: &mut Snapshot) {
        entity.files.insert(self.path.to_owned(), self.file_id.to_owned());
        entity.event_id = self.event_id;
    }
    
    fn event_id(&self) -> &str {
        &self.event_id
    }
}

impl Into<EventKind> for AddFileEvent {
    fn into(self) -> EventKind {
        EventKind::AddFile(self)
    }
}