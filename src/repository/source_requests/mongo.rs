use futures::TryStreamExt;
use mongodb::bson::{self, doc};
use serde::Deserialize;

use crate::repository::{error::QueryError, mongo::MongoDatabase};

use super::{Approvable, Completable, CreateSourceRequest, SourceRequest, SourceRequestRepository, SourceRequestSummary};

impl SourceRequestRepository for MongoDatabase {
    async fn get_by_id<'a>(&self, id: &'a str) -> Result<Option<SourceRequest>, QueryError> {
        #[derive(Deserialize)]
        struct Model {
            source_request: SourceRequest,
        }

        self.connection_pool
            .get()
            .await?
            .collection::<Model>("source_requests")
            .find_one(doc! { "id": id })
            .projection(doc! { "source_request": 1, })
            .await
            .map(|result| result.map(|model| model.source_request))
            .map_err(QueryError::from)
    }

    async fn get_approvable<'a>(&self, id: &'a str) -> Result<Option<Approvable>, QueryError> {
        #[derive(Deserialize)]
        struct Model {
            approvable: Approvable,
        }

        self.connection_pool
            .get()
            .await?
            .collection::<Model>("source_requests")
            .find_one(doc! { "id": id })
            .projection(doc! { "source_request": 1 })
            .await
            .map(|result| result.map(|model| model.approvable))
            .map_err(QueryError::from)
    }

    async fn get_completable<'a>(&self, id: &'a str) -> Result<Option<Completable>, QueryError> {
        #[derive(Deserialize)]
        struct Model {
            completable: Completable,
        }

        self.connection_pool
            .get()
            .await?
            .collection::<Model>("source_requests")
            .find_one(doc! { "id": id })
            .projection(doc! { "source_request": 1 })
            .await
            .map(|result| result.map(|model| model.completable))
            .map_err(QueryError::from)
    }

    async fn create<'a>(&self, id: &'a str, source_request: impl Into<CreateSourceRequest<'a>>) -> Result<(), QueryError> {
        let source_request = source_request.into();

        self.connection_pool
            .get()
            .await?
            .collection("source_requests")
            .insert_one(doc! {
                "id": id,
                "project_id": source_request.project_id(),
                "source_request": bson::to_bson(&source_request)?,
            })
            .await
            .map(|_| ())
            .map_err(QueryError::from)
    }

    async fn update<'a>(&self, id: &'a str, source_request: impl Into<SourceRequest>) -> Result<(), QueryError> {
        self.connection_pool
            .get()
            .await?
            .collection::<()>("source_requests")
            .update_one(
                doc! { "id": id }, 
                doc! { "$set": { 
                    "source_request": bson::to_bson(&source_request.into())? 
                } },
            )
            .await
            .map(|_| ())
            .map_err(QueryError::from)
    }

    async fn list_by_project_id(&self, project_id: &str) -> Result<Vec<(String, SourceRequestSummary)>, QueryError> {
        #[derive(Deserialize)]
        struct Model {
            id: String,
            source_request: SourceRequestSummary,
        }

        let mut cursor = self.connection_pool
            .get()
            .await?
            .collection::<Model>("source_requests")
            .find(doc! { "project_id": project_id })
            .projection(doc! { 
                "id": 1,
                "source_request": 1 
            })
            .await?;

        let mut source_requests = Vec::new();

        while let Some(model) = cursor.try_next().await? {
            source_requests.push((model.id, model.source_request));
        }

        Ok(source_requests)
    }

    async fn delete<'a>(&self, id: &'a str) -> Result<(), QueryError> {
        self.connection_pool
            .get()
            .await?
            .collection::<()>("source_requests")
            .delete_one(doc! { "id": id })
            .await
            .map(|_| ())
            .map_err(QueryError::from)
    }
}