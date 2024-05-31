use axum::response::IntoResponse;
use axum::http::StatusCode;
use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct ValidationError {
    pub r: String,
}

impl From<&str> for ValidationError {
    fn from(value: &str) -> Self {
        ValidationError { r: value.to_string() }
    }
}

impl<'a> IntoResponse for ValidationError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::BAD_REQUEST, Json(ValidationError::from(self))).into_response()
    }
}

pub trait Validate {
    fn validate(&self) -> Result<(), ValidationError>;
}