use axum::{routing::{get, post}, Router};

use super::web::{add_files, create_project, get_project_by_id};

pub trait ProjectsRouter {
    fn register_projects_routes(self) -> Self;
}

impl ProjectsRouter for Router {
    fn register_projects_routes(self) -> Self {
        self.route("/projects", post(create_project))
            .route("/projects/:id", get(get_project_by_id))
            .route("/projects/:id/files", post(add_files))
    }
}