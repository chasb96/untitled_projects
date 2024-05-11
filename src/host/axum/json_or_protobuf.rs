use axum::http::{header::ACCEPT, HeaderMap};
use log::error;

const CONTENT_TYPE_PROTOBUF: &'static str = "application/octet-stream";
const CONTENT_TYPE_JSON: &'static str = "application/json";

pub enum JsonOrProtobuf<T> {
    Protobuf(T),
    Json(T),
}

impl<T> JsonOrProtobuf<T> {
    pub fn new(body: T, content_type: &str) -> Self {
        match content_type {
            CONTENT_TYPE_PROTOBUF => Self::Protobuf(body),
            CONTENT_TYPE_JSON => Self::Json(body),
            _ => {
                error!("Attempted to serialize invalid Content-Type {}", content_type);
                panic!("Attempted to serialize invalid Content-Type {}", content_type)
            },
        }
    }

    pub fn decompose(self) -> (T, String) {
        match self {
            JsonOrProtobuf::Protobuf(body) => (body, CONTENT_TYPE_PROTOBUF.to_string()),
            JsonOrProtobuf::Json(body) => (body, CONTENT_TYPE_JSON.to_string()),
        }
    }
}

impl<T> From<(T, String)> for JsonOrProtobuf<T> {
    fn from(value: (T, String)) -> Self {
        Self::new(value.0, &value.1)
    }
}

impl<T> From<(T, HeaderMap)> for JsonOrProtobuf<T> {
    fn from(value: (T, HeaderMap)) -> Self {
        let accept = value.1
            .get(ACCEPT)
            .and_then(|header_value| header_value.to_str().ok());

        if accept == Some(CONTENT_TYPE_PROTOBUF) {
            Self::Protobuf(value.0)
        } else {
            Self::Json(value.0)
        }
    }
}

impl<T> Into<(T, String)> for JsonOrProtobuf<T> {
    fn into(self) -> (T, String) {
        self.decompose()
    }
}