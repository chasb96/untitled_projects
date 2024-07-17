use futures::TryStreamExt;
use mongodb::bson::{self, doc};
use serde::Deserialize;

use crate::{events::Snapshot, repository::{error::QueryError, mongo::MongoDatabase}};

use super::SnapshotsRepository;

impl SnapshotsRepository for MongoDatabase {
    async fn list(&self, project_ids: &Option<Vec<String>>) -> Result<Vec<Snapshot>, QueryError> {
        let conn = self.connection_pool
            .get()
            .await?;

        conn.collection::<Snapshot>("snapshots")
            .find(match project_ids {
                Some(project_ids) => doc! {
                    "project_id": { "$in": project_ids },
                },
                None => doc! {},
            })
            .await?
            .try_collect()
            .await
            .map_err(QueryError::from)
    }

    async fn create(&self, project_id: &str, version: &str, snapshot: impl Into<Snapshot>) -> Result<(), QueryError> {
        let conn = self.connection_pool
            .get()
            .await?;

        let snapshot = snapshot.into();

        conn.collection("snapshots")
            .insert_one(doc! {
                "project_id": project_id,
                "version": version,
                "snapshot": bson::to_bson(&snapshot)?,
            })
            .await?;

        Ok(())
    }

    async fn get_by_id(&self, project_id: &str, version: &str) -> Result<Option<Snapshot>, QueryError> {
        let conn = self.connection_pool
            .get()
            .await?;

        #[derive(Deserialize)]
        struct Model {
            snapshot: Snapshot,
        }

        conn.collection::<Model>("snapshots")
            .find_one(doc! {
                "project_id": project_id,
                "version": version,
            })
            .projection(doc! { "snapshot": 1, })
            .await
            .map(|result| result.map(|model| model.snapshot))
            .map_err(QueryError::from)
    }

    async fn delete(&self, project_id: &str, version: &str) -> Result<(), QueryError> {
        let conn = self.connection_pool
            .get()
            .await?;

        conn.collection::<()>("snapshots")
            .delete_one(doc! {
                "project_id": project_id,
                "version": version,
            })
            .await?;

        Ok(())
    }
}