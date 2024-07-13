use search_client::{CreateProjectRequest, SearchClient};

use super::{error::HandleError, Message, Queueable};

pub struct CreateProject {
    pub project_id: String,
    pub name: String,
}

impl Queueable for CreateProject {
    async fn handle(self) -> Result<(), HandleError> {
        SearchClient::default()
            .create_project(CreateProjectRequest {
                project_id: self.project_id,
                project_name: self.name.clone(),
                value: self.name,
            })
            .await?;

        Ok(())
    }
}

impl Into<Message> for CreateProject {
    fn into(self) -> Message {
        Message::CreateProject(self)
    }
}