use futures::TryStreamExt;
use mongodb::bson::{self, doc};
use serde::Deserialize;

use crate::{events::Snapshot, repository::{error::QueryError, mongo::MongoDatabase}};

use super::{ListQuery, SnapshotsRepository};

impl SnapshotsRepository for MongoDatabase {
    async fn list(&self, query: &ListQuery) -> Result<Vec<Snapshot>, QueryError> {
        let conn = self.connection_pool
            .get()
            .await?;

        #[derive(Deserialize)]
        struct Model {
            snapshot: Snapshot,
        }

        conn.collection::<Model>("snapshots")
            .find(match query {
                ListQuery::ProjectIds { project_ids } => doc! { "p": { "$in": project_ids } },
                ListQuery::UserId { user_id } => doc! { "s.uid": user_id },
            })
            .await?
            .try_collect()
            .await
            .map(|models: Vec<Model>| models
                .into_iter()
                .map(|model| model.snapshot)
                .collect()
            )
            .map_err(QueryError::from)
    }

    async fn create(&self, project_id: &str, version: &str, snapshot: impl Into<Snapshot>) -> Result<(), QueryError> {
        let conn = self.connection_pool
            .get()
            .await?;

        let snapshot = snapshot.into();

        conn.collection("snapshots")
            .insert_one(doc! {
                "p": project_id,
                "v": version,
                "s": bson::to_bson(&snapshot)?,
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
                "p": project_id,
                "v": version,
            })
            .projection(doc! { "s": 1, })
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
                "p": project_id,
                "v": version,
            })
            .await?;

        Ok(())
    }
}