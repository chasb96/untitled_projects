use futures::TryStreamExt;
use mongodb::bson::{self, doc};

use crate::repository::{error::QueryError, mongo::MongoDatabase};

use super::{Approvable, Completable, CreateSourceRequest, SourceRequest, SourceRequestRepository, SourceRequestSummary};

impl SourceRequestRepository for MongoDatabase {
    async fn get_by_id<'a>(&self, id: &'a str) -> Result<Option<SourceRequest>, QueryError> {
        self.connection_pool
            .get()
            .await?
            .collection::<SourceRequest>("source_requests")
            .find_one(doc! { "id": id })
            .projection(doc! { "source_request": 1, })
            .await
            .map_err(QueryError::from)
    }

    async fn get_approvable<'a>(&self, id: &'a str) -> Result<Option<Approvable>, QueryError> {
        self.connection_pool
            .get()
            .await?
            .collection::<Approvable>("source_requests")
            .find_one(doc! { "id": id })
            .projection(doc! { "source_request": 1 })
            .await
            .map_err(QueryError::from)
    }

    async fn get_completable<'a>(&self, id: &'a str) -> Result<Option<Completable>, QueryError> {
        self.connection_pool
            .get()
            .await?
            .collection::<Completable>("source_requests")
            .find_one(doc! { "id": id })
            .projection(doc! { "source_request": 1 })
            .await
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
                doc! { "$set": { "source_request": bson::to_bson(&source_request.into())? } },
            )
            .await
            .map(|_| ())
            .map_err(QueryError::from)
    }

    async fn list_by_project_id(&self, project_id: &str) -> Result<Vec<(String, SourceRequestSummary)>, QueryError> {
        let mut cursor = self.connection_pool
            .get()
            .await?
            .collection::<(String, SourceRequestSummary)>("source_requests")
            .find(doc! { "project_id": project_id })
            .projection(doc! { 
                "id": 1,
                "source_request": 1 
            })
            .await?;

        let mut source_requests = Vec::new();

        while let Some(source_request) = cursor.try_next().await? {
            source_requests.push((source_request.0, source_request.1));
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