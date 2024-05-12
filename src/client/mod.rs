mod response;
mod error;
pub mod axum;

use std::env;

use prost::Message;
pub use response::ProjectResponse;
pub use error::Error;

use reqwest::{header::CONTENT_TYPE, Client, StatusCode};

pub struct ProjectsClient {
    http_client: Client,
    base_url: String,
}

impl ProjectsClient {
    pub fn new(http_client: Client, base_url: String) -> Self {
        Self {
            http_client,
            base_url,
        }
    }

    pub async fn get_project_by_id(&self, project_id: &str) -> Result<ProjectResponse, Error> {
        let response = self.http_client
            .get(format!("{}/projects/{}", self.base_url, project_id))
            .header(CONTENT_TYPE, "application/octet-stream")
            .send()
            .await?
            .error_for_status()?;
        
        let response_bytes = response.bytes().await?;

        let response = ProjectResponse::decode(response_bytes)?;

        Ok(response)
    }
}

impl Default for ProjectsClient {
    fn default() -> Self {
        let base_url = env::var("PROJECTS_BASE_URL")
            .unwrap_or("http://projects".to_string())
            .trim_end_matches('/')
            .to_string();

        Self { 
            http_client: Default::default(),
            base_url
        }
    }
}