use serde::Deserialize;

use super::{ApprovedSourceRequest, CompletedSourceRequest};

#[derive(Deserialize)]
pub enum Completable {
    Approved(ApprovedSourceRequest),
}

impl Completable {
    pub fn complete(self) -> CompletedSourceRequest {
        match self {
            Completable::Approved(approved) => approved.complete(),
        }
    }
}