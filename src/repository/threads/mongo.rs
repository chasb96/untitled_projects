use futures::TryStreamExt;
use mongodb::bson::doc;
use serde::Deserialize;

use crate::repository::{error::QueryError, mongo::MongoDatabase};

use super::{Comment, NewComment, NewThread, Thread, ThreadsRepository};

impl ThreadsRepository for MongoDatabase {
    async fn create<'a>(&self, thread: NewThread<'a>) -> Result<(), QueryError> {
        let conn = self.connection_pool
            .get()
            .await?;

        #[derive(Deserialize)]
        struct Order {
            order: u32,
        }

        let order = conn.collection::<Order>("project_thread")
            .find(doc! { "project_id": thread.project_id, })
            .max(doc! { "order": 1, })
            .await?
            .try_next()
            .await?
            .map(|count| count.order);

        conn.collection("project_thread")
            .insert_one(doc! {
                "id": thread.id,
                "project_id": thread.project_id,
                "user_id": thread.user_id,
                "title": thread.title,
                "order": if let Some(order) = order { order + 1 } else { 0 },
            })
            .await
            .map(|_| ())
            .map_err(QueryError::from)
    }

    async fn list(&self, project_id: &str) -> Result<Vec<Thread>, QueryError> {
        let mut cursor = self.connection_pool
            .get()
            .await?
            .collection::<Thread>("project_thread")
            .find(doc! { "project_id": project_id, })
            .sort(doc! { "order": 1, })
            .await?;

        let mut threads = Vec::new();

        while let Some(thread) = cursor.try_next().await? {
            threads.push(thread);
        }

        Ok(threads)
    }

    async fn get_by_id(&self, id: &str) -> Result<Option<Thread>, QueryError> {
        self.connection_pool
            .get()
            .await?
            .collection::<Thread>("project_thread")
            .find_one(doc! { "id": id, })
            .await
            .map_err(QueryError::from)
    }

    async fn create_comment<'a>(&self, comment: NewComment<'a>) -> Result<(), QueryError> {
        let conn = self.connection_pool
            .get()
            .await?;

        #[derive(Deserialize)]
        struct Order {
            order: u32,
        }

        let order = conn.collection::<Order>("project_thread_comments")
            .find(doc! { "thread_id": comment.thread_id, })
            .sort(doc! { "order": -1 })
            .projection(doc! { "order": 1, })
            .limit(1)
            .await?
            .try_next()
            .await?
            .map(|count| count.order);

        conn.collection("project_thread_comments")
            .insert_one(doc! {
                "id": comment.id,
                "thread_id": comment.thread_id,
                "user_id": comment.user_id,
                "content": comment.content,
                "order": if let Some(order) = order { order + 1 } else { 0 },
            })
            .await
            .map(|_| ())
            .map_err(QueryError::from)
    }

    async fn list_comments(&self, thread_id: &str) -> Result<Vec<Comment>, QueryError> {
        let mut cursor = self.connection_pool
            .get()
            .await?
            .collection::<Comment>("project_thread_comments")
            .find(doc! { "thread_id": thread_id, })
            .await?;

        let mut comments = Vec::new();

        while let Some(comment) = cursor.try_next().await? {
            comments.push(comment);
        }

        Ok(comments)
    }
}