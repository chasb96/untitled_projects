use std::collections::HashMap;

use prost::Message;

#[derive(Message)]
pub struct ProjectResponse {
    #[prost(string, tag = "1")]
    pub id: String,
    #[prost(string, tag = "2")]
    pub name: String,
    #[prost(int32, tag = "3")]
    pub user_id: i32,
    #[prost(map= "string, string", tag = "4")]
    pub files: HashMap<String, String>,
}
