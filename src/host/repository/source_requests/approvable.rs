use serde::Deserialize;

use super::{approved::ApprovedSourceRequest, new::NewSourceRequest};

#[derive(Deserialize)]
pub enum Approvable {
    New(NewSourceRequest),
    Approved(ApprovedSourceRequest),
}

impl Approvable {
    pub fn approve(self, approver: i32) -> ApprovedSourceRequest {
        match self {
            Approvable::New(new) => new.approve(approver),
            Approvable::Approved(approved) => approved.approve(approver),
        }
    }
}