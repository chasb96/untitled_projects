use deadpool::managed::{Manager, Metrics, RecycleError, RecycleResult};
use redis::{Client, RedisError};
use redis::aio::MultiplexedConnection;
use mongodb::{bson::doc, Client as MongoClient};

pub struct MongoConnectionManager {
    pub client: MongoClient,
    pub database: String,
}

impl Manager for MongoConnectionManager {
    type Type = mongodb::Database;
    type Error = mongodb::error::Error;
    
    async fn create(&self) -> Result<mongodb::Database, Self::Error> {
        Ok(self.client.database(&self.database))
    }
    
    async fn recycle(&self, conn: &mut mongodb::Database, _: &Metrics) -> RecycleResult<Self::Error> {
        conn.run_command(doc! { "ping": 1 })
            .await
            .map(|_| ())
            .map_err(|_| RecycleError::message("Failed to ping mongodb"))
    }
}

pub struct RedisConnectionManager {
    pub client: Client,
}

impl Manager for RedisConnectionManager {
    type Type = MultiplexedConnection;
    type Error = RedisError;
    
    async fn create(&self) -> Result<MultiplexedConnection, Self::Error> {
        self.client
            .get_multiplexed_async_connection()
            .await
    }
    
    async fn recycle(&self, conn: &mut MultiplexedConnection, _: &Metrics) -> RecycleResult<Self::Error> {
        redis::cmd("PING")
            .query_async(conn)
            .await
            .map_err(|_| RecycleError::message("Failed to ping redis"))
    }
}