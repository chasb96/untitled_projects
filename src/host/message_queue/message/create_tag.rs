use crate::host::repository::tags::{TagsRepository, TagsRepositoryOption};

use super::{error::HandleError, Message, Queueable};

pub struct CreateTag {
    pub project_id: String,
    pub tag: String,
}

impl Queueable for CreateTag {
    async fn handle(self) -> Result<(), HandleError> {
        let tags_repository = TagsRepositoryOption::default();
        
        let tags = tags_repository
            .list(&self.project_id)
            .await?;

        if !tags.contains(&self.tag) {
            tags_repository
                .create(&self.project_id, &self.tag)
                .await?;
        }

        Ok(())
    }
}

impl Into<Message> for CreateTag {
    fn into(self) -> Message {
        Message::CreateTag(self)
    }
}