use serde::Serialize;

use self::{
    add_file_event::AddFileEvent, 
    create_event::CreateEvent, 
    name_event::NameEvent, 
    remove_file_event::RemoveFileEvent, 
    set_owner_event::SetOwnerEvent, 
    snapshot::Snapshot
};

mod snapshot;
mod name_event;
mod set_owner_event;
mod remove_file_event;
mod add_file_event;
mod create_event;

#[derive(Serialize)]
pub enum EventKind {
    Create(CreateEvent),
    Name(NameEvent),
    SetOwner(SetOwnerEvent),
    AddFile(AddFileEvent),
    RemoveFile(RemoveFileEvent),
}

pub trait Event<T> {
    fn apply(&self, entity: &mut T);
}

impl Event<Snapshot> for EventKind {
    fn apply(&self, entity: &mut Snapshot) {
        match self {
            EventKind::Create(e) => e.apply(entity),
            EventKind::Name(e) => e.apply(entity),
            EventKind::SetOwner(e) => e.apply(entity),
            EventKind::AddFile(e) => e.apply(entity),
            EventKind::RemoveFile(e) => e.apply(entity),
        }
    }
}