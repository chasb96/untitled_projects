use serde::{Deserialize, Serialize};

mod snapshot;
mod name_event;
mod set_owner_event;
mod remove_files_event;
mod add_files_event;
mod create_event;
mod rename_files_event;
mod source_request_append;

pub use name_event::NameEvent;
pub use set_owner_event::SetOwnerEvent;
pub use remove_files_event::RemoveFilesEvent;
pub use add_files_event::AddFilesEvent;
pub use add_files_event::FileMap;
pub use create_event::CreateEvent;
pub use snapshot::Snapshot;
pub use rename_files_event::RenameFilesEvent;
pub use source_request_append::SourceRequestAppend;

use super::repository;

#[derive(Serialize, Deserialize, Clone)]
pub enum EventKind {
    #[serde(rename = "c")]
    Create(CreateEvent),
    #[serde(rename = "n")]
    Name(NameEvent),
    #[serde(rename = "so")]
    SetOwner(SetOwnerEvent),
    #[serde(rename = "af")]
    AddFiles(AddFilesEvent),
    #[serde(rename = "rm")]
    RemoveFiles(RemoveFilesEvent),
    #[serde(rename = "mv")]
    RenameFiles(RenameFilesEvent),
    #[serde(rename = "sra")]
    SourceRequestAppend(SourceRequestAppend),
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
            EventKind::SourceRequestAppend(e) => e.apply(entity),
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
            EventKind::SourceRequestAppend(e) => e.event_id(),
        }
    }
}

impl From<repository::source_requests::CompletedSourceRequest> for EventKind {
    fn from(source_request: repository::source_requests::CompletedSourceRequest) -> Self {
        EventKind::SourceRequestAppend(SourceRequestAppend::from(source_request))
    }
}