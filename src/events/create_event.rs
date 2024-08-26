use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use super::{snapshot::Snapshot, Event, EventKind};

#[derive(Serialize, Deserialize, Clone)]
pub struct CreateEvent {
    #[serde(rename = "eid")]
    pub event_id: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "n")]
    pub name: String,
    #[serde(rename = "uid")]
    pub owner_id: String,
}

impl Event for CreateEvent {
    fn apply(self, entity: &mut Snapshot) {
        *entity = self.into();
    }
    
    fn event_id(&self) -> &str {
        &self.event_id
    }

    fn previous(&self) -> Option<&str> {
        None
    }
}

impl Into<Snapshot> for CreateEvent {
    fn into(self) -> Snapshot {
        Snapshot {
            id: self.id,
            name: self.name,
            user_id: self.owner_id,
            event_id: self.event_id,
            files: HashMap::new(),
        }
    }
}

impl Into<EventKind> for CreateEvent {
    fn into(self) -> EventKind {
        EventKind::Create(self)
    }
}