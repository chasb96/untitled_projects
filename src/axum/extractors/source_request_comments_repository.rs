use std::ops::Deref;
use axum::{async_trait, extract::FromRequestParts, http::{request::Parts, StatusCode}};

use crate::repository::source_requests::comments::SourceRequestCommentRepositoryOption;

pub struct SourceRequestCommentsRepositoryExtractor(SourceRequestCommentRepositoryOption);

impl Deref for SourceRequestCommentsRepositoryExtractor {
    type Target = SourceRequestCommentRepositoryOption;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for SourceRequestCommentsRepositoryExtractor {
    fn default() -> Self {
        Self(Default::default())
    }
}

#[async_trait]
impl<T> FromRequestParts<T> for SourceRequestCommentsRepositoryExtractor {
    type Rejection = StatusCode;

    async fn from_request_parts<'a, 'b>(_: &'a mut Parts, _: &'b T) -> Result<Self, Self::Rejection> {
        Ok(SourceRequestCommentsRepositoryExtractor::default())
    }
}