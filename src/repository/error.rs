use std::{error::Error, fmt::Display};
use deadpool::managed::PoolError;
use prost::DecodeError;
use redis::RedisError;
use mongodb::error::Error as MongoError;
use mongodb::bson::ser::Error as MongoSerializeError;
use mongodb::bson::de::Error as MongoDeserializeError;

#[derive(Debug)]
pub enum QueryError {
    MongoPool(PoolError<MongoError>),
    RedisPool(PoolError<RedisError>),
    ProtobufDecode(DecodeError),
    Redis(RedisError),
    Mongo(MongoError),
    MongoSerialize(MongoSerializeError),
    MongoDeserialize(MongoDeserializeError),
}

impl Error for QueryError { }

impl Display for QueryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QueryError::MongoPool(e) => write!(f, "Error obtaining connection from mongo pool: {}", e),
            QueryError::RedisPool(e) => write!(f, "Error obtaining connection from redis pool: {}", e),
            QueryError::ProtobufDecode(e) => write!(f, "Error decoding protobuf: {}", e),
            QueryError::Redis(e) => write!(f, "Error accessing cache: {}", e),
            QueryError::Mongo(e) => write!(f, "Error accessing mongo: {}", e),
            QueryError::MongoSerialize(e) => write!(f, "Error serializing to bson: {}", e),
            QueryError::MongoDeserialize(e) => write!(f, "Error deserializing from bson: {}", e),
        }
    }
}

impl From<PoolError<RedisError>> for QueryError {
    fn from(value: PoolError<RedisError>) -> Self {
        QueryError::RedisPool(value)
    }
}

impl From<DecodeError> for QueryError {
    fn from(value: DecodeError) -> Self {
        QueryError::ProtobufDecode(value)
    }
}

impl From<RedisError> for QueryError {
    fn from(value: RedisError) -> Self {
        QueryError::Redis(value)
    }
}

impl From<PoolError<MongoError>> for QueryError {
    fn from(value: PoolError<MongoError>) -> Self {
        QueryError::MongoPool(value)
    }
}

impl From<MongoError> for QueryError {
    fn from(value: MongoError) -> Self {
        QueryError::Mongo(value)
    }
}

impl From<MongoSerializeError> for QueryError {
    fn from(value: MongoSerializeError) -> Self {
        QueryError::MongoSerialize(value)
    }
}

impl From<MongoDeserializeError> for QueryError {
    fn from(value: MongoDeserializeError) -> Self {
        QueryError::MongoDeserialize(value)
    }
}