use std::ops::Deref;
use axum::{async_trait, extract::FromRequestParts, http::{request::Parts, StatusCode}};

use crate::repository::threads::ThreadsRepositoryOption;

pub struct ThreadsRepositoryExtractor(ThreadsRepositoryOption);

impl Deref for ThreadsRepositoryExtractor {
    type Target = ThreadsRepositoryOption;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for ThreadsRepositoryExtractor {
    fn default() -> Self {
        Self(Default::default())
    }
}

#[async_trait]
impl<T> FromRequestParts<T> for ThreadsRepositoryExtractor {
    type Rejection = StatusCode;

    async fn from_request_parts<'a, 'b>(_: &'a mut Parts, _: &'b T) -> Result<Self, Self::Rejection> {
        Ok(ThreadsRepositoryExtractor::default())
    }
}