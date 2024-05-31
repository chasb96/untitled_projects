use self::error::HandleError;

use super::ProjectViewed;

mod error;
pub mod project_viewed;

pub enum Message {
    ProjectViewed(ProjectViewed)
}

impl Message {
    pub async fn handle(self) -> Result<(), HandleError> {
        match self {
            Message::ProjectViewed(m) => m.handle().await 
        }
    }
}