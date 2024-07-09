use std::ops::Deref;
use axum::{async_trait, extract::FromRequestParts, http::{request::Parts, StatusCode}};

use crate::repository::tags::TagsRepositoryOption;

pub struct TagsRepositoryExtractor(TagsRepositoryOption);

impl Deref for TagsRepositoryExtractor {
    type Target = TagsRepositoryOption;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for TagsRepositoryExtractor {
    fn default() -> Self {
        Self(Default::default())
    }
}

#[async_trait]
impl<T> FromRequestParts<T> for TagsRepositoryExtractor {
    type Rejection = StatusCode;

    async fn from_request_parts<'a, 'b>(_: &'a mut Parts, _: &'b T) -> Result<Self, Self::Rejection> {
        Ok(TagsRepositoryExtractor::default())
    }
}