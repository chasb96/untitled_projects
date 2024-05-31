use self::error::HandleError;

use super::{AssignProject, CreateSnapshot, CreateTag, ProjectViewed, RemoveTag};

mod error;
pub mod project_viewed;
pub mod create_tag;
pub mod create_snapshot;
pub mod remove_tag;
pub mod assign_project;

pub enum Message {
    ProjectViewed(ProjectViewed),
    CreateTag(CreateTag),
    RemoveTag(RemoveTag),
    CreateSnapshot(CreateSnapshot),
    AssignProject(AssignProject),
}

pub trait Queueable {
    async fn handle(self) -> Result<(), HandleError>;
}

impl Queueable for Message {
    async fn handle(self) -> Result<(), HandleError> {
        match self {
            Message::ProjectViewed(m) => m.handle().await,
            Message::CreateTag(m) => m.handle().await,
            Message::CreateSnapshot(m) => m.handle().await,
            Message::RemoveTag(m) => m.handle().await, 
            Message::AssignProject(m) => m.handle().await,
        }
    }
}