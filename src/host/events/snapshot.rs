use std::collections::HashMap;
use super::Event;

pub struct Snapshot {
    pub id: String,
    pub name: String,
    pub user_id: i32,
    pub event_id: String,
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

    pub fn apply_event(&mut self, event: impl Event<Self>) {
        event.apply(self)
    }

    pub fn apply_events(&mut self, events: impl Iterator<Item = impl Event<Self>>) {
        for event in events {
            self.apply_event(event)
        }
    }
}