use futures::TryStreamExt;
use mongodb::bson::{self, doc};
use serde::Deserialize;

use crate::{events::EventKind, repository::{error::QueryError, mongo::MongoDatabase}};
use crate::events::Event;

use super::EventsRepository;

impl EventsRepository for MongoDatabase {
    async fn create(&self, project_id: &str, event: impl Into<EventKind>) -> Result<(), QueryError> {
        let conn = self.connection_pool
            .get()
            .await?;

        let order = conn.collection::<u32>("events")
            .find(doc! { "project_id": project_id, })
            .sort(doc! { "order": -1 })
            .projection(doc! { "order": 1, })
            .limit(1)
            .await?
            .try_next()
            .await?;

        let event: EventKind = event.into();

        conn.collection("events")
            .insert_one(doc! {
                "project_id": project_id,
                "event_id": event.event_id(),
                "order": if let Some(order) = order { order + 1 } else { 0 },
                "event": bson::to_bson(&event)?,
            })
            .await?;

        Ok(())
    }

    async fn list(&self, project_id: &str) -> Result<Vec<EventKind>, QueryError> {
        let conn = self.connection_pool
            .get()
            .await?;

        #[derive(Deserialize)]
        struct Model {
            event: EventKind,
        }

        let mut cursor = conn.collection::<Model>("events")
            .find(doc! { "project_id": project_id, })
            .sort(doc! { "order": 1, })
            .projection(doc! { "event": 1, })
            .await?;

        let mut events = Vec::new();

        while let Some(model) = cursor.try_next().await? {
            events.push(model.event);
        }

        Ok(events)
    }

    async fn list_until(&self, project_id: &str, event_id: &str) -> Result<Vec<EventKind>, QueryError> {
        let mut found = false;

        let events = self.list(project_id)
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