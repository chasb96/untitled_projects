use std::ops::Deref;
use axum::{async_trait, extract::FromRequestParts, http::{request::Parts, StatusCode}};

use crate::repository::source_requests::SourceRequestRepositoryOption;

pub struct SourceRequestsRepositoryExtractor(SourceRequestRepositoryOption);

impl Deref for SourceRequestsRepositoryExtractor {
    type Target = SourceRequestRepositoryOption;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for SourceRequestsRepositoryExtractor {
    fn default() -> Self {
        Self(Default::default())
    }
}

#[async_trait]
impl<T> FromRequestParts<T> for SourceRequestsRepositoryExtractor {
    type Rejection = StatusCode;

    async fn from_request_parts<'a, 'b>(_: &'a mut Parts, _: &'b T) -> Result<Self, Self::Rejection> {
        Ok(SourceRequestsRepositoryExtractor::default())
    }
}