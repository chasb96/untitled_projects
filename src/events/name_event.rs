use serde::{Deserialize, Serialize};

use super::{snapshot::Snapshot, Event, EventKind};

#[derive(Serialize, Deserialize, Clone)]
pub struct NameEvent {
    #[serde(rename = "eid")]
    pub event_id: String,
    #[serde(rename = "n")]
    pub name: String,
    #[serde(rename = "pe")]
    pub previous_event_id: String,
}

impl Event for NameEvent {
    fn apply(self, entity: &mut Snapshot) {
        entity.name = self.name.to_owned();
        entity.event_id = self.event_id;
    }

    fn event_id(&self) -> &str {
        &self.event_id
    }

    fn previous(&self) -> Option<&str> {
        Some(&self.previous_event_id)
    }
}

impl Into<EventKind> for NameEvent {
    fn into(self) -> EventKind {
        EventKind::Name(self)
    }
}