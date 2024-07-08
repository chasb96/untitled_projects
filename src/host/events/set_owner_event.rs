use serde::{Deserialize, Serialize};

use super::{snapshot::Snapshot, Event, EventKind};

#[derive(Serialize, Deserialize, Clone)]
pub struct SetOwnerEvent {
    #[serde(rename = "eid")]
    pub event_id: String,
    #[serde(rename = "uid")]
    pub owner_id: String,
}

impl Event for SetOwnerEvent {
    fn apply(self, entity: &mut Snapshot) {
        entity.user_id = self.owner_id;
        entity.event_id = self.event_id;
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