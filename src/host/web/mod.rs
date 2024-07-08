mod create_project;
mod get_project_by_id;
mod event;
mod events;
mod list_projects;
mod create_tag;
mod list_tags;
mod remove_tag;
mod search_projects;
mod threads;
mod source_requests;

pub mod validate;

pub use create_project::create_project;
pub use get_project_by_id::get_project_by_id;
pub use event::event;
pub use list_projects::list_projects;
pub use create_tag::create_tag;
pub use list_tags::list_tags;
pub use remove_tag::remove_tag;
pub use search_projects::search_projects;

pub use threads::create_thread;
pub use threads::get_thread_by_id;
pub use threads::list_threads;
pub use threads::create_comment;

pub use source_requests::create_source_request;
pub use source_requests::list_source_requests_by_project;
pub use source_requests::get_source_request;
pub use source_requests::approve_source_request;
pub use source_requests::complete_source_request;
pub use source_requests::source_request_diff;

pub use source_requests::comments::create_source_request_comment;
pub use source_requests::comments::list_source_request_comments;

use axum::http::StatusCode;

type ApiResult<T> = Result<T, StatusCode>;