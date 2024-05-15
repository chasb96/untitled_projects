use serde::{Deserialize, Serialize};

use super::{snapshot::Snapshot, Event, EventKind};

#[derive(Serialize, Deserialize)]
pub struct NameEvent {
    pub event_id: String,
    pub name: String,
}

impl Event for NameEvent {
    fn apply(&self, entity: &mut Snapshot) {
        entity.name = self.name.to_owned()
    }

    fn event_id(&self) -> &str {
        &self.event_id
    }
}

impl Into<EventKind> for NameEvent {
    fn into(self) -> EventKind {
        EventKind::Name(self)
    }
}