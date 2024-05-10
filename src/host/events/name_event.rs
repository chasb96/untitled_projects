use serde::Serialize;

use super::{snapshot::Snapshot, Event};

#[derive(Serialize)]
pub struct NameEvent {
    pub name: String,
}

impl Event<Snapshot> for NameEvent {
    fn apply(&self, entity: &mut Snapshot) {
        entity.name = self.name.to_owned()
    }
}