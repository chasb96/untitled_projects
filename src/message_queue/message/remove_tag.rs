use crate::repository::{search::{SearchRepository, SearchRepositoryOption}, tags::{TagsRepository, TagsRepositoryOption}};

use super::{Message, Queueable};

pub struct RemoveTag {
    pub project_id: String,
    pub tag: String,
}

impl Queueable for RemoveTag {
    async fn handle(self) -> Result<(), super::error::HandleError> {
        let tags_repository = TagsRepositoryOption::default();
        
        tags_repository
            .delete(&self.project_id, &self.tag)
            .await?;

        SearchRepositoryOption::default()
            .delete(&self.project_id, &self.tag)
            .await?;

        Ok(())
    }
}

impl Into<Message> for RemoveTag {
    fn into(self) -> Message {
        Message::RemoveTag(self)
    }
}