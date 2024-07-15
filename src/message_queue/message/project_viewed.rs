use metrics_client::{MetricsClient, ViewProjectRequest};

use super::{error::HandleError, Message, Queueable};

pub struct ProjectViewed {
    pub id: String,
}

impl Queueable for ProjectViewed {
    async fn handle(self) -> Result<(), HandleError> {
        MetricsClient::default()
            .view_project(ViewProjectRequest {
                project_id: self.id,
             })
             .await
             .map_err(HandleError::from)
    }
}

impl Into<Message> for ProjectViewed {
    fn into(self) -> Message {
        Message::ProjectViewed(self)
    }
}