use axum::{routing::{delete, get, post, put}, Router};

use super::web::{create_comment, create_project, create_tag, create_thread, event, get_project_by_id, get_thread_by_id, list_projects, list_tags, list_threads, remove_tag, search_projects};

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
    }
}