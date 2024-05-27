use crate::host::repository::metrics::{MetricsRepository, MetricsRepositoryOption};

use super::{error::HandleError, Message};

pub struct ProjectViewed {
    pub id: String,
}

impl ProjectViewed {
    pub async fn handle(&self) -> Result<(), HandleError> {
        MetricsRepositoryOption::default()
            .increment_view_count(&self.id)
            .await
            .map_err(HandleError::from)
    }
}

impl Into<Message> for ProjectViewed {
    fn into(self) -> Message {
        Message::ProjectViewed(self)
    }
}