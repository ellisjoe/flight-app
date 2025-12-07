use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    InvalidArgument(String),

    #[error("{0}")]
    NotFound(String),

    #[error("{0}")]
    InternalError(String),
}

impl Error {
    pub fn invalid_argument(msg: &str) -> Self {
        Error::InvalidArgument(msg.to_string())
    }

    pub fn not_found(msg: &str) -> Self {
        Error::NotFound(msg.to_string())
    }

    pub fn internal_error(msg: &str) -> Self {
        Error::InternalError(msg.to_string())
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::InvalidArgument(e) => (StatusCode::BAD_REQUEST, e).into_response(),
            Error::NotFound(e) => (StatusCode::NOT_FOUND, e).into_response(),
            Error::InternalError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
        }
    }
}
