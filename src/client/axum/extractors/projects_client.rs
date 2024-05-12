use std::{ops::Deref, sync::OnceLock};
use axum::{async_trait, extract::FromRequestParts, http::{request::Parts, StatusCode}};

use crate::client::ProjectsClient as ClientInner;

static PROJECTS_CLIENT: OnceLock<ClientInner> = OnceLock::new();

pub struct ProjectsClient(pub &'static ClientInner);

impl Deref for ProjectsClient {
    type Target = ClientInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[async_trait]
impl<T> FromRequestParts<T> for ProjectsClient {
    type Rejection = StatusCode;

    async fn from_request_parts<'a, 'b>(_: &'a mut Parts, _: &'b T) -> Result<Self, Self::Rejection> {
        let client = PROJECTS_CLIENT.get_or_init(ClientInner::default);

        Ok(ProjectsClient(client))
    }
}