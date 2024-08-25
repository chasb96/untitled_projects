use mongodb::bson::doc;

use futures::TryStreamExt;
use serde::Deserialize;
use crate::repository::mongo::MongoDatabase;
use crate::repository::error::QueryError;

use super::SourceRequestCommentRepository;
use super::SourceRequestComment;
use super::CreateSourceRequestComment;

impl SourceRequestCommentRepository for MongoDatabase {
    async fn create<'a>(&self, source_request_comment: CreateSourceRequestComment<'a>) -> Result<(), QueryError> {
        self.connection_pool
            .get()
            .await?
            .collection("source_request_comments")
            .insert_one(doc! {
                "id": source_request_comment.id,
                "source_request_id": source_request_comment.source_request_id,
                "user_id": source_request_comment.user_id,
                "content": source_request_comment.content,
            })
            .await
            .map(|_| ())
            .map_err(QueryError::from)
    }

    async fn list<'a>(&self, source_request_id: &'a str) -> Result<Vec<SourceRequestComment>, QueryError> {
        #[derive(Deserialize)]
        struct Model {
            id: String,
            source_request_id: String,
            user_id: String,
            content: String,
        }

        let mut cursor = self.connection_pool
            .get()
            .await?
            .collection::<Model>("source_request_comments")
            .find(doc! { "source_request_id": source_request_id })
            .projection(doc! {
                "source_request_id": 1,
                "user_id": 1,
                "content": 1,
            })
            .await?;

        let mut source_request_comments = Vec::new();

        while let Some(model) = cursor.try_next().await? {
            source_request_comments.push(SourceRequestComment {
                id: model.id,
                source_request_id: model.source_request_id,
                user_id: model.user_id,
                content: model.content,
            });
        }

        Ok(source_request_comments)
    }
}