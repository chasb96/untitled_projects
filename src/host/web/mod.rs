mod create_project;
mod get_project_by_id;
mod add_files;

pub use create_project::create_project;
pub use get_project_by_id::get_project_by_id;
pub use add_files::add_files;

use axum::http::StatusCode;

type ApiResult<T> = Result<T, StatusCode>;