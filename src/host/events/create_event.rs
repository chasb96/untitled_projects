use serde::Serialize;

use super::{snapshot::Snapshot, Event};

#[derive(Serialize)]
pub struct CreateEvent;

impl Event<Snapshot> for CreateEvent {
    fn apply(&self, entity: &mut Snapshot) {
        *entity =  Snapshot::new()
    }
}