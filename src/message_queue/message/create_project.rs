use crate::repository::search::{SearchRepository, SearchRepositoryOption};

use super::{error::HandleError, Message, Queueable};

pub struct CreateProject {
    pub project_id: String,
    pub name: String,
}

impl Queueable for CreateProject {
    async fn handle(self) -> Result<(), HandleError> {
        SearchRepositoryOption::default()
            .create(&self.project_id, &self.name, &self.name)
            .await?;

        Ok(())
    }
}

impl Into<Message> for CreateProject {
    fn into(self) -> Message {
        Message::CreateProject(self)
    }
}