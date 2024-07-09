use crate::repository::{search::{SearchRepository, SearchRepositoryOption}, snapshots::{SnapshotsRepository, SnapshotsRepositoryOption}, tags::{TagsRepository, TagsRepositoryOption}};

use super::{error::HandleError, Message, Queueable};

pub struct CreateTag {
    pub project_id: String,
    pub tag: String,
}

impl Queueable for CreateTag {
    async fn handle(self) -> Result<(), HandleError> {
        let project = SnapshotsRepositoryOption::default()
            .get_by_id(&self.project_id, "latest")
            .await?
            .ok_or(HandleError::NotFoundError(self.project_id))?;

        let tags_repository = TagsRepositoryOption::default();
        
        let tags = tags_repository
            .list(&project.id)
            .await?;

        if !tags.contains(&self.tag) {
            tags_repository
                .create(&project.id, &self.tag)
                .await?;
        }

        SearchRepositoryOption::default()
            .create(&project.id, &project.name, &self.tag)
            .await?;

        Ok(())
    }
}

impl Into<Message> for CreateTag {
    fn into(self) -> Message {
        Message::CreateTag(self)
    }
}