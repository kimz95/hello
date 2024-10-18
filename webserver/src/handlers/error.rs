use actix_web::ResponseError;
use thiserror::Error;
use crate::auth::error::AuthError;
use crate::database::error::DatabaseError;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Database error occurred")]
    DatabaseApiError(#[from] DatabaseError),
    #[error("You are not authorized to perform this action")]
    AuthorizationError(#[from] AuthError),
}

impl ResponseError for ApiError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match *self {
            ApiError::DatabaseApiError(ref err) => err.status_code(),
            ApiError::AuthorizationError(ref err) => err.status_code(),
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        match *self {
            ApiError::DatabaseApiError(ref err) => err.error_response(),
            ApiError::AuthorizationError(ref err) => err.error_response(),
        }
    }
}

impl From<DatabaseError> for diesel::result::Error {
    fn from(error: DatabaseError) -> Self {
        // diesel::result::Error::DatabaseError(diesel::result::DatabaseErrorKind::UnableToSendCommand, Box::new(error.to_string()))
        diesel::result::Error::NotFound
    }
}

impl From<DatabaseError> for AuthError {
    fn from(value: DatabaseError) -> Self {
        AuthError::InvalidCredentials
    }
}

impl From<actix_web::Error> for ApiError {
    fn from(value: actix_web::Error) -> Self {
        ApiError::AuthorizationError(AuthError::InvalidCredentials)
    }
}