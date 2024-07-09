use axum::{async_trait, extract::{rejection::JsonRejection, FromRequest, Request}, response::IntoResponse, Json, RequestExt};
use serde::{de::DeserializeOwned, Serialize};
use axum::http::StatusCode;

use crate::web::validate::{self, Validate};

pub struct Validated<T>(pub T);

#[derive(Serialize)]
pub struct Reason {
    #[serde(rename = "r")]
    pub reason: String,
}

pub enum ValidationError {
    Validation(Reason),
    Json(JsonRejection),
}

impl IntoResponse for ValidationError {
    fn into_response(self) -> axum::response::Response {
        match self {
            ValidationError::Validation(reason) => (StatusCode::BAD_REQUEST, Json(reason)).into_response(),
            ValidationError::Json(rejection) => rejection.into_response(),
        }
    }
}

impl From<JsonRejection> for ValidationError {
    fn from(rejection: JsonRejection) -> Self {
        ValidationError::Json(rejection)
    }
}

impl From<validate::ValidationError> for ValidationError {
    fn from(error: validate::ValidationError) -> Self {
        ValidationError::Validation(Reason { reason: error.r })
    }
}

#[async_trait]
impl<T, S> FromRequest<S> for Validated<Json<T>>
where
    T: 'static,
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = ValidationError;

    async fn from_request(request: Request, _: &S) -> Result<Self, Self::Rejection> {
        let Json(payload) = request
            .extract::<Json<T>, _>()
            .await?;

        payload.validate()?;

        Ok(Validated(Json(payload)))
    }
}