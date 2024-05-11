use serde::{Deserialize, Serialize};

use super::{snapshot::Snapshot, Event};

#[derive(Serialize, Deserialize)]
pub struct AddFileEvent {
    pub event_id: String,
    pub path: String,
    pub file_id: String,
}

impl Event<Snapshot> for AddFileEvent {
    fn apply(&self, entity: &mut Snapshot) {
        entity.files.insert(self.path.to_owned(), self.file_id.to_owned());
    }
    
    fn event_id(&self) -> &str {
        &self.event_id
    }
}