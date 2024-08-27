use futures::TryStreamExt;
use mongodb::bson::Document;
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

    async fn list(&self, project_id: &str, event_id: &str) -> Result<Vec<EventKind>, QueryError> {
        let conn = self.connection_pool
            .get()
            .await?;

        #[derive(Deserialize)]
        struct Model {
            #[serde(rename = "c")]
            event: EventKind,
        }

        let mut cursor = conn.collection::<Document>("events")
            .aggregate([
                doc! { "$match": {  "p": project_id,  "e": event_id } },
                doc! { 
                    "$graphLookup": {
                        "from": "events",
                        "startWith": "$pe",
                        "connectFromField": "pe",
                        "connectToField": "e",
                        "as": "history",
                        "restrictSearchWithMatch": { "p": project_id },
                        "maxDepth": 32,
                    }
                },
                doc! { "$unwind": "$history" },
                doc! { "$replaceRoot": { "newRoot": "$history" } },
                doc! { "$project": { "c": 1 } },
            ])
            .await?;

        let mut events = Vec::new();

        while let Some(document) = cursor.try_next().await? {
            let model: Model = bson::from_document(document)?;

            events.push(model.event);
        }

        events.reverse();

        Ok(events)
    }
}