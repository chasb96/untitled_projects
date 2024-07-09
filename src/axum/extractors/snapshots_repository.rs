use std::ops::Deref;
use axum::{async_trait, extract::FromRequestParts, http::{request::Parts, StatusCode}};

use crate::repository::snapshots::SnapshotsRepositoryOption;

pub struct SnapshotsRepositoryExtractor(SnapshotsRepositoryOption);

impl Deref for SnapshotsRepositoryExtractor {
    type Target = SnapshotsRepositoryOption;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for SnapshotsRepositoryExtractor {
    fn default() -> Self {
        Self(Default::default())
    }
}

#[async_trait]
impl<T> FromRequestParts<T> for SnapshotsRepositoryExtractor {
    type Rejection = StatusCode;

    async fn from_request_parts<'a, 'b>(_: &'a mut Parts, _: &'b T) -> Result<Self, Self::Rejection> {
        Ok(SnapshotsRepositoryExtractor::default())
    }
}