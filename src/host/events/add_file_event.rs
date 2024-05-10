use serde::Serialize;

use super::{snapshot::Snapshot, Event};

#[derive(Serialize)]
pub struct AddFileEvent {
    pub path: String,
    pub file_id: String,
}

impl Event<Snapshot> for AddFileEvent {
    fn apply(&self, entity: &mut Snapshot) {
        entity.files.insert(self.path.to_owned(), self.file_id.to_owned());
    }
}