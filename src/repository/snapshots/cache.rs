use prost::Message;
use redis::AsyncCommands;

use crate::{events::Snapshot, repository::{error::QueryError, redis::RedisCache}};

use super::{ListQuery, SnapshotsRepository};

pub struct SnapshotsCachingRepository<T> {
    cache: RedisCache,
    repository: T,
}

impl<T> SnapshotsRepository for SnapshotsCachingRepository<T> 
where
    T: SnapshotsRepository,
{
    async fn list(&self, query: &ListQuery) -> Result<Vec<Snapshot>, QueryError> {
        self.repository
            .list(query)
            .await
    }

    async fn create(&self, project_id: &str, version: &str, snapshot: impl Into<Snapshot>) -> Result<(), QueryError> {
        self.repository
            .create(project_id, version, snapshot)
            .await
    }

    async fn get_by_id(&self, project_id: &str, version: &str) -> Result<Option<Snapshot>, QueryError> {
        let cache_key = format!("project:{}:{}", project_id, version);

        let mut conn = self.cache
            .connection_pool
            .get()
            .await?;

        if let Some(bytes) = conn.get(&cache_key).await? {
            let bytes: Vec<u8> = bytes;

            return Ok(Some(Snapshot::decode::<&[u8]>(&bytes)?))
        }

        if let Some(user) = self.repository.get_by_id(project_id, version).await? {
            let _: () = conn.set(cache_key, user.encode_to_vec()).await?;

            return Ok(Some(user))
        }

        Ok(None)
    }

    async fn delete(&self, project_id: &str, version: &str) -> Result<(), QueryError> {
        self.repository
            .delete(project_id, version)
            .await?;
        
        let cache_key = format!("project:{}:{}", project_id, version);

        let mut conn = self.cache
            .connection_pool
            .get()
            .await?;

        let _: () = conn.del(&cache_key).await?;

        Ok(())
    }
}

impl<T> Default for SnapshotsCachingRepository<T> 
where
    T: Default,
{
    fn default() -> Self {
        Self {
            cache: RedisCache::default(),
            repository: T::default(),
        }
    }
}