use serde::{Deserialize, Serialize};

use super::{snapshot::Snapshot, Event, EventKind};

#[derive(Serialize, Deserialize, Clone)]
pub struct AddFilesEvent {
    pub event_id: String,
    pub files: Vec<FileMap>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FileMap {
    pub path: String,
    pub file_id: String,
}

impl Event for AddFilesEvent {
    fn apply(self, entity: &mut Snapshot) {
        for file in self.files {
            entity.files.insert(file.path, file.file_id);
        }
        
        entity.event_id = self.event_id;
    }
    
    fn event_id(&self) -> &str {
        &self.event_id
    }
}

impl Into<EventKind> for AddFilesEvent {
    fn into(self) -> EventKind {
        EventKind::AddFile(self)
    }
}