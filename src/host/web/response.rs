use prost::Message;
use serde::Serialize;

#[derive(Serialize)]
pub struct CreateProjectResponse {
    pub id: String,
}

#[derive(Serialize, Message)]
pub struct ProjectResponse {
    #[prost(string, tag = "1")]
    pub id: String,
    #[prost(string, tag = "2")]
    pub name: String,
    #[prost(int32, tag = "3")]
    pub user_id: i32,
}
