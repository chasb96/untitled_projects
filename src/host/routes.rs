use axum::{routing::{delete, get, post, put}, Router};

use super::web::*;

pub trait ProjectsRouter {
    fn register_projects_routes(self) -> Self;
}

impl ProjectsRouter for Router {
    fn register_projects_routes(self) -> Self {
        self.route("/projects", get(list_projects))
            .route("/projects", post(create_project))
            .route("/projects/search", get(search_projects))
            .route("/projects/:project_id", get(get_project_by_id))
            .route("/projects/:project_id", put(event))
            .route("/projects/:project_id/tags", get(list_tags))
            .route("/projects/:project_id/tags", post(create_tag))
            .route("/projects/:project_id/tags/:tag", delete(remove_tag))
            .route("/projects/:project_id/threads", get(list_threads))
            .route("/projects/:project_id/threads", post(create_thread))
            .route("/projects/:project_id/threads/:thread_id", get(get_thread_by_id))
            .route("/projects/:project_id/threads/:thread_id/comments", post(create_comment))
            .route("/projects/:project_id/source_requests", get(list_source_requests_by_project))
            .route("/projects/:project_id/source_requests", post(create_source_request))
            .route("/projects/:project_id/source_requests/:source_request_id", get(get_source_request))
            .route("/projects/:project_id/source_requests/:source_request_id/approve", post(approve_source_request))
            .route("/projects/:project_id/source_requests/:source_request_id/complete", post(complete_source_request))
    }
}