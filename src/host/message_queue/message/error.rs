use std::{error::Error, fmt::{self, Display}};

use crate::host::repository::error::QueryError;

#[derive(Debug)]
pub enum HandleError {
    QueryError(QueryError),
    UserClientError(users::client::Error),
}

impl Error for HandleError { }

impl Display for HandleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error handling message: ")?;

        match self {
            Self::QueryError(e) => write!(f, "QueryError({})", e),
            Self::UserClientError(e) => write!(f, "UserClientError({})", e),
        }
    }
}

impl From<QueryError> for HandleError {
    fn from(value: QueryError) -> Self {
        HandleError::QueryError(value)
    }
}

impl From<users::client::Error> for HandleError {
    fn from(value: users::client::Error) -> Self {
        HandleError::UserClientError(value)
    }
}