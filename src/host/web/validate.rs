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

pub trait Validate {
    fn validate(&self) -> Result<(), ValidationError>;
}