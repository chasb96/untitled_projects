mod add_files_request;
mod remove_files_request;
mod rename_files_request;

pub use add_files_request::AddFilesRequest;
pub use remove_files_request::RemoveFilesRequest;
pub use rename_files_request::RenameFilesRequest;

use serde::Deserialize;

use crate::host::events::EventKind;

#[derive(Deserialize)]
pub enum EventRequest {
    #[serde(rename = "af")]
    AddFiles(AddFilesRequest),
    #[serde(rename = "rm")]
    RemoveFiles(RemoveFilesRequest),
    #[serde(rename = "mv")]
    RenameFiles(RenameFilesRequest),
}

impl Into<EventKind> for EventRequest {
    fn into(self) -> EventKind {
        match self {
            EventRequest::AddFiles(request) => EventKind::AddFiles(request.into()),
            EventRequest::RemoveFiles(request) => EventKind::RemoveFiles(request.into()),
            EventRequest::RenameFiles(request) => EventKind::RenameFiles(request.into()),
        }
    }
}