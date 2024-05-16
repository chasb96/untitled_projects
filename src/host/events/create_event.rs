use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use super::{snapshot::Snapshot, Event, EventKind};

#[derive(Serialize, Deserialize, Clone)]
pub struct CreateEvent {
    pub event_id: String,
    pub id: String,
    pub name: String,
    pub owner_id: i32,
}

impl Event for CreateEvent {
    fn apply(self, entity: &mut Snapshot) {
        *entity = self.into();
    }
    
    fn event_id(&self) -> &str {
        &self.event_id
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