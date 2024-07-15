use futures::TryStreamExt;
use mongodb::bson::doc;
use serde::Deserialize;

use crate::repository::{error::QueryError, mongo::MongoDatabase};

use super::TagsRepository;

impl TagsRepository for MongoDatabase {
    async fn list(&self, project_id: &str) -> Result<Vec<String>, QueryError> {
        #[derive(Deserialize)]
        struct Model {
            tag: String,
        }

        let mut cursor = self.connection_pool
            .get()
            .await?
            .collection::<Model>("project_tags")
            .find(doc! { "project_id": project_id })
            .projection(doc! { "tag": 1 })
            .await?;

        let mut tags = Vec::new();

        while let Some(model) = cursor.try_next().await? {
            tags.push(model.tag);
        }

        Ok(tags)
    }

    async fn create(&self, project_id: &str, tag: &str) -> Result<(), QueryError> {
        self.connection_pool
            .get()
            .await?
            .collection("project_tags")
            .insert_one(doc! {
                "project_id": project_id,
                "tag": tag,
            })
            .await
            .map(|_| ())
            .map_err(QueryError::from)
    }

    async fn delete(&self, project_id: &str, tag: &str) -> Result<(), QueryError> {
        self.connection_pool
            .get()
            .await?
            .collection::<()>("project_tags")
            .delete_one(doc! {
                "project_id": project_id,
                "tag": tag,
            })
            .await
            .map(|_| ())
            .map_err(QueryError::from)
    }
}