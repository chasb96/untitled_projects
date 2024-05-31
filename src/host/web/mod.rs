mod create_project;
mod get_project_by_id;
mod event;
mod events;
mod list_projects;
mod create_tag;
mod list_tags;
mod remove_tag;

pub mod validate;

pub use create_project::create_project;
pub use get_project_by_id::get_project_by_id;
pub use event::event;
pub use list_projects::list_projects;
pub use create_tag::create_tag;
pub use list_tags::list_tags;
pub use remove_tag::remove_tag;

use axum::http::StatusCode;

type ApiResult<T> = Result<T, StatusCode>;