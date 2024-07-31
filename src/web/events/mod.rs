mod add_files_request;
mod remove_files_request;
mod rename_files_request;

use std::error::Error;

pub use add_files_request::AddFilesRequest;
use prost::Message;
pub use remove_files_request::RemoveFilesRequest;
pub use rename_files_request::RenameFilesRequest;

use crate::events::EventKind;

use super::validate::{Validate, ValidationError};

#[derive(Message)]
pub struct EventRequestMessage {
    #[prost(message, optional, tag = "1")]
    add_files: Option<AddFilesRequest>,
    #[prost(message, optional, tag = "2")]
    remove_files: Option<RemoveFilesRequest>,
    #[prost(message, optional, tag = "3")]
    rename_files: Option<RenameFilesRequest>,
}

#[derive(Debug)]
pub struct InvalidEventRequestMessageError { }

impl Error for InvalidEventRequestMessageError { }

impl std::fmt::Display for InvalidEventRequestMessageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid EventRequest: only one event request can be specified")
    }
}

impl TryInto<EventRequest> for EventRequestMessage {
    type Error = InvalidEventRequestMessageError;

    fn try_into(self) -> Result<EventRequest, Self::Error> {
        match (self.add_files, self.remove_files, self.rename_files) {
            (Some(add_files), None, None) => Ok(EventRequest::AddFiles(add_files)),
            (None, Some(remove_files), None) => Ok(EventRequest::RemoveFiles(remove_files)),
            (None, None, Some(rename_files)) => Ok(EventRequest::RenameFiles(rename_files)),
            _ => Err(InvalidEventRequestMessageError { })
        }
    }
}

pub enum EventRequest {
    AddFiles(AddFilesRequest),
    RemoveFiles(RemoveFilesRequest),
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

impl Validate for EventRequest {
    fn validate(&self) -> Result<(), ValidationError> {
        match self {
            EventRequest::AddFiles(request) => request.validate(),
            EventRequest::RemoveFiles(request) => request.validate(),
            EventRequest::RenameFiles(request) => request.validate(),
        }
    }
}