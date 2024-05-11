mod response;
mod error;

use prost::Message;
pub use response::ProjectResponse;
pub use error::Error;

use reqwest::{header::CONTENT_TYPE, Client};

pub struct ProjectsClient {
    http_client: Client,
}

impl ProjectsClient {
    pub fn new(http_client: Client) -> Self {
        Self {
            http_client
        }
    }

    pub async fn get_project_by_id(&self, project_id: String) -> Result<ProjectResponse, Error> {
        let response = self.http_client
            .post(format!("http://projects/projects/{}", project_id))
            .header(CONTENT_TYPE, "application/octet-stream")
            .send()
            .await?;

        let response_bytes = response.bytes().await?;

        let response = ProjectResponse::decode(response_bytes)?;

        Ok(response)
    }
}

impl Default for ProjectsClient {
    fn default() -> Self {
        Self { 
            http_client: Default::default() 
        }
    }
}