use serde::Serialize;

use super::{snapshot::Snapshot, Event};

#[derive(Serialize)]
pub struct RemoveFileEvent {
    pub path: String,
}

impl Event<Snapshot> for RemoveFileEvent {
    fn apply(&self, entity: &mut Snapshot) {
        entity.files.remove(&self.path);
    }
}