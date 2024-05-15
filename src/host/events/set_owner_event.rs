use serde::{Deserialize, Serialize};

use super::{snapshot::Snapshot, Event, EventKind};

#[derive(Serialize, Deserialize)]
pub struct SetOwnerEvent {
    pub event_id: String,
    pub owner_id: i32,
}

impl Event for SetOwnerEvent {
    fn apply(&self, entity: &mut Snapshot) {
        entity.user_id = self.owner_id
    }

    fn event_id(&self) -> &str {
        &self.event_id
    }
}

impl Into<EventKind> for SetOwnerEvent {
    fn into(self) -> EventKind {
        EventKind::SetOwner(self)
    }
}