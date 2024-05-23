use axum::{routing::{get, post, put}, Router};

use super::web::{create_project, event, get_project_by_id};

pub trait ProjectsRouter {
    fn register_projects_routes(self) -> Self;
}

impl ProjectsRouter for Router {
    fn register_projects_routes(self) -> Self {
        self.route("/projects", post(create_project))
            .route("/projects/:id", get(get_project_by_id))
            .route("/projects/:id/files", put(event))
    }
}