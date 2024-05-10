use serde::Serialize;

use super::{snapshot::Snapshot, Event};

#[derive(Serialize)]
pub struct SetOwnerEvent {
    pub owner_id: i32,
}

impl Event<Snapshot> for SetOwnerEvent {
    fn apply(&self, entity: &mut Snapshot) {
        entity.user_id = self.owner_id
    }
}