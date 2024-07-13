use search_client::SearchClient;

use crate::repository::tags::{TagsRepository, TagsRepositoryOption};

use super::{error::HandleError, Message, Queueable};

pub struct RemoveTag {
    pub project_id: String,
    pub tag: String,
}

impl Queueable for RemoveTag {
    async fn handle(self) -> Result<(), HandleError> {
        let tags_repository = TagsRepositoryOption::default();
        
        tags_repository
            .delete(&self.project_id, &self.tag)
            .await?;

        SearchClient::default()
            .delete_project_value(&self.project_id, &self.tag)
            .await?;

        Ok(())
    }
}

impl Into<Message> for RemoveTag {
    fn into(self) -> Message {
        Message::RemoveTag(self)
    }
}