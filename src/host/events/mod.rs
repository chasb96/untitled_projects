use serde::{Deserialize, Serialize};

mod snapshot;
mod name_event;
mod set_owner_event;
mod remove_files_event;
mod add_files_event;
mod create_event;
mod rename_files_event;

pub use name_event::NameEvent;
pub use set_owner_event::SetOwnerEvent;
pub use remove_files_event::RemoveFilesEvent;
pub use add_files_event::AddFilesEvent;
pub use add_files_event::FileMap;
pub use create_event::CreateEvent;
pub use snapshot::Snapshot;
pub use rename_files_event::RenameFilesEvent;

#[derive(Serialize, Deserialize, Clone)]
pub enum EventKind {
    Create(CreateEvent),
    Name(NameEvent),
    SetOwner(SetOwnerEvent),
    AddFiles(AddFilesEvent),
    RemoveFiles(RemoveFilesEvent),
    RenameFiles(RenameFilesEvent),
}

pub trait Event {
    fn apply(self, entity: &mut Snapshot);

    fn event_id(&self) -> &str;
}

impl Event for EventKind {
    fn apply(self, entity: &mut Snapshot) {
        match self {
            EventKind::Create(e) => e.apply(entity),
            EventKind::Name(e) => e.apply(entity),
            EventKind::SetOwner(e) => e.apply(entity),
            EventKind::AddFiles(e) => e.apply(entity),
            EventKind::RemoveFiles(e) => e.apply(entity),
            EventKind::RenameFiles(e) => e.apply(entity),
        }
    }
    
    fn event_id(&self) -> &str {
        match self {
            EventKind::Create(e) => e.event_id(),
            EventKind::Name(e) => e.event_id(),
            EventKind::SetOwner(e) => e.event_id(),
            EventKind::AddFiles(e) => e.event_id(),
            EventKind::RemoveFiles(e) => e.event_id(),
            EventKind::RenameFiles(e) => e.event_id(),
        }
    }
}