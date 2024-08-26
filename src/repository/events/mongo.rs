use futures::TryStreamExt;
use mongodb::bson::{self, doc};
use serde::Deserialize;

use crate::repository::mongo::MongoDatabase;
use crate::repository::error::QueryError;
use crate::events::EventKind;
use crate::events::Event;

use super::EventsRepository;

impl EventsRepository for MongoDatabase {
    async fn create(&self, project_id: &str, event: impl Into<EventKind>) -> Result<(), QueryError> {
        let event: EventKind = event.into();

        let conn = self.connection_pool
            .get()
            .await?;

        conn.collection("events")
            .insert_one(doc! {
                "p": project_id,
                "e": event.event_id(),
                "pe": event.previous(),
                "c": bson::to_bson(&event)?,
            })
            .await
            .map(|_| ())
            .map_err(QueryError::from)
    }

    async fn list(&self, project_id: &str) -> Result<Vec<EventKind>, QueryError> {
        let conn = self.connection_pool
            .get()
            .await?;

        #[derive(Deserialize)]
        struct Model {
            event: EventKind,
        }

        conn.collection::<Model>("events")
            .find(doc! { "p": project_id, })
            .projection(doc! { "c": 1, })
            .await?
            .try_collect()
            .await
            .map(|models: Vec<Model>| models
                .into_iter()
                .map(|model| model.event)
                .collect()
            )
            .map_err(QueryError::from)
    }

    async fn list_until(&self, project_id: &str, event_id: &str) -> Result<Vec<EventKind>, QueryError> {
        let mut found = false;

        let events = self
            .list(project_id)
            .await?
            .into_iter()
            .take_while(|event| match found {
                true => false,
                false => { found = event.event_id() == event_id; true }
            })
            .collect();

        Ok(events)
    }
}