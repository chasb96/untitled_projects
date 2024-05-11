use serde::{Deserialize, Serialize};

use super::{snapshot::Snapshot, Event};

#[derive(Serialize, Deserialize)]
pub struct RemoveFileEvent {
    pub event_id: String,
    pub path: String,
}

impl Event<Snapshot> for RemoveFileEvent {
    fn apply(&self, entity: &mut Snapshot) {
        entity.files.remove(&self.path);
    }

    fn event_id(&self) -> &str {
        &self.event_id
    }
}