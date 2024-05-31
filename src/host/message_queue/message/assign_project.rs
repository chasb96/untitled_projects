use users::client::{ProjectRequest, UsersClient};

use super::{error::HandleError, Message, Queueable};

pub struct AssignProject {
    pub user_id: i32,
    pub project_id: String,
}

impl Queueable for AssignProject {
    async fn handle(self) -> Result<(), HandleError> {
        let add_project_request = ProjectRequest {
            project_id: self.project_id,
        };
    
        UsersClient::default()
            .add_project(self.user_id, add_project_request)
            .await?;

        Ok(())
    }
}

impl Into<Message> for AssignProject {
    fn into(self) -> Message {
        Message::AssignProject(self)
    }
}