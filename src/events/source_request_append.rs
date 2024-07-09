use rand::distributions::{Alphanumeric, DistString};
use serde::{Deserialize, Serialize};

use crate::repository::{self, EVENT_ID_LENGTH};

use super::{Event, EventKind, Snapshot};

#[derive(Serialize, Deserialize, Clone)]
pub struct SourceRequestAppend {
    #[serde(rename = "eid")]
    pub event_id: String,
    #[serde(rename = "u")]
    pub user_id: String,
    #[serde(rename = "t")]
    pub title: String,
    #[serde(rename = "f")]
    pub files: Vec<FileMap>,
}

impl From<repository::source_requests::CompletedSourceRequest> for SourceRequestAppend {
    fn from(source_request: repository::source_requests::CompletedSourceRequest) -> Self {
        SourceRequestAppend {
            event_id: Alphanumeric.sample_string(&mut rand::thread_rng(), EVENT_ID_LENGTH),
            user_id: source_request.user_id,
            title: source_request.title,
            files: source_request.files
                .into_iter()
                .map(FileMap::from)
                .collect()
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FileMap {
    #[serde(rename = "p")]
    pub path: String,
    #[serde(rename = "fid")]
    pub file_id: String,
}

impl From<repository::source_requests::FileMap> for FileMap {
    fn from(file_map: repository::source_requests::FileMap) -> Self {
        FileMap {
            path: file_map.path,
            file_id: file_map.file_id,
        }
    }
}

impl Event for SourceRequestAppend {
    fn apply(self, entity: &mut Snapshot) {
        for file in self.files {
            if entity.files.contains_key(&file.path) {
                entity.files.insert(file.path, file.file_id);
            }
        }
        
        entity.event_id = self.event_id;
    }
    
    fn event_id(&self) -> &str {
        &self.event_id
    }
}

impl Into<EventKind> for SourceRequestAppend {
    fn into(self) -> EventKind {
        EventKind::SourceRequestAppend(self)
    }
}