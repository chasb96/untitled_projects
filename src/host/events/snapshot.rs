use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use super::Event;

#[derive(Serialize, Deserialize)]
pub struct Snapshot {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "n")]
    pub name: String,
    #[serde(rename = "uid")]
    pub user_id: i32,
    #[serde(rename = "eid")]
    pub event_id: String,
    #[serde(rename = "f")]
    pub files: HashMap<String, String>,
}

impl Snapshot {
    pub fn new() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            user_id: 0,
            event_id: String::new(),
            files: HashMap::new(),
        }
    }

    pub fn apply_event(&mut self, event: impl Event) {
        event.apply(self)
    }

    pub fn apply_events(&mut self, events: impl Iterator<Item = impl Event>) {
        for event in events {
            self.apply_event(event)
        }
    }
}

impl<T, S> From<T> for Snapshot 
where
    T: Iterator<Item = S>,
    S: Event
{
    fn from(events: T) -> Self {
        let mut snapshot = Snapshot::new();

        snapshot.apply_events(events);
        
        snapshot
    }
}