use serde::{Deserialize, Serialize};

use super::{snapshot::Snapshot, Event, EventKind};

#[derive(Serialize, Deserialize, Clone)]
pub struct AddFilesEvent {
    #[serde(rename = "eid")]
    pub event_id: String,
    #[serde(rename = "f")]
    pub files: Vec<FileMap>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FileMap {
    #[serde(rename = "p")]
    pub path: String,
    #[serde(rename = "fid")]
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
        EventKind::AddFiles(self)
    }
}