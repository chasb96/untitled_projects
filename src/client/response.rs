use prost::Message;

#[derive(Message)]
pub struct ProjectResponse {
    #[prost(string, tag = "1")]
    pub id: String,
    #[prost(string, tag = "2")]
    pub name: String,
    #[prost(int32, tag = "3")]
    pub user_id: i32,
    #[prost(message, repeated, tag = "4")]
    pub files: Vec<ProjectFileResponse>,
}

#[derive(Message)]
pub struct ProjectFileResponse {
    #[prost(string, tag = "1")]
    pub id: String,
    #[prost(string, tag = "2")]
    pub name: String,
}

#[derive(Message)]
pub struct SearchResponse {
    #[prost(message, repeated, tag = "1")]
    pub records: Vec<SearchRecord>,
}

#[derive(Message)]
pub struct SearchRecord {
    #[prost(string, tag = "1")]
    pub project_id: String,
    #[prost(string, tag = "2")]
    pub name: String,
    #[prost(float, tag = "3")]
    pub score: f32,
}