use std::{error::Error, fmt::{self, Display}};

use crate::repository::error::QueryError;

#[derive(Debug)]
pub enum HandleError {
    QueryError(QueryError),
    UserClientError(users_client::Error),
    SearchClientError(search_client::Error),
    NotFoundError(String),
}

impl Error for HandleError { }

impl Display for HandleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error handling message: ")?;

        match self {
            Self::QueryError(e) => write!(f, "QueryError({})", e),
            Self::UserClientError(e) => write!(f, "UserClientError({})", e),
            Self::SearchClientError(e) => write!(f, "SearchClientError({})", e),
            Self::NotFoundError(e) => write!(f, "NotFoundError({})", e),
        }
    }
}

impl From<QueryError> for HandleError {
    fn from(value: QueryError) -> Self {
        HandleError::QueryError(value)
    }
}

impl From<users_client::Error> for HandleError {
    fn from(value: users_client::Error) -> Self {
        HandleError::UserClientError(value)
    }
}

impl From<search_client::Error> for HandleError {
    fn from(value: search_client::Error) -> Self {
        HandleError::SearchClientError(value)
    }
}