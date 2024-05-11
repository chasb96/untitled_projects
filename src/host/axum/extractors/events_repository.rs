use std::ops::Deref;
use axum::{async_trait, extract::FromRequestParts, http::{request::Parts, StatusCode}};

use crate::host::repository::events::EventsRepositoryOption;

pub struct EventsRepositoryExtractor(EventsRepositoryOption);

impl Deref for EventsRepositoryExtractor {
    type Target = EventsRepositoryOption;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for EventsRepositoryExtractor {
    fn default() -> Self {
        Self(Default::default())
    }
}

#[async_trait]
impl<T> FromRequestParts<T> for EventsRepositoryExtractor {
    type Rejection = StatusCode;

    async fn from_request_parts<'a, 'b>(_: &'a mut Parts, _: &'b T) -> Result<Self, Self::Rejection> {
        Ok(EventsRepositoryExtractor::default())
    }
}