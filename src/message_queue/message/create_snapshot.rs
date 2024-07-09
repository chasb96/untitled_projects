use crate::{events::Snapshot, repository::snapshots::{SnapshotsRepository, SnapshotsRepositoryOption}};

use super::{error::HandleError, Message, Queueable};

pub struct CreateSnapshot {
    pub project_id: String,
    pub version: String,
    pub snapshot: Snapshot,
}

impl Queueable for CreateSnapshot 
{
    async fn handle(self) -> Result<(), HandleError> {
        let snapshots_repository = SnapshotsRepositoryOption::default();

        snapshots_repository
            .delete(&self.project_id, &self.version)
            .await?;

        snapshots_repository
            .create(&self.project_id, &self.version, self.snapshot)
            .await?;

        Ok(())
    }
}

impl Into<Message> for CreateSnapshot {
    fn into(self) -> Message {
        Message::CreateSnapshot(self)
    }
}