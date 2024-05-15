use ::axum::{routing::get, Router};
use log_layer::LogLayer;
use routes::ProjectsRouter;

mod axum;
mod repository;
mod web;
mod configuration;
mod health;
mod routes;
mod events;

pub fn router() -> Router {
    Router::new()
        .route("/health", get(health::health))
        .register_projects_routes()
        .layer(LogLayer::new())
}