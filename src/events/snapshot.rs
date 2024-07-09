use std::collections::HashMap;
use prost::Message;
use serde::{Deserialize, Serialize};

use super::Event;

#[derive(Serialize, Deserialize, Message)]
pub struct Snapshot {
    #[serde(rename = "id")]
    #[prost(string, tag = "1")]
    pub id: String,
    #[serde(rename = "n")]
    #[prost(string, tag = "2")]
    pub name: String,
    #[serde(rename = "uid")]
    #[prost(string, tag = "3")]
    pub user_id: String,
    #[serde(rename = "eid")]
    #[prost(string, tag = "4")]
    pub event_id: String,
    #[serde(rename = "f")]
    #[prost(map = "string, string", tag = "5")]
    pub files: HashMap<String, String>,
}

impl Snapshot {
    pub fn new() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            user_id: String::new(),
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