use aws_sdk_s3::{error::SdkError, operation::get_object::GetObjectError, Error as S3Error};
use axum::response::{IntoResponse, Response};
use hyper::StatusCode;
use std::error::Error;

#[derive(Debug)]
pub struct AppError {
    status_code: StatusCode,
    details: String,
}

impl AppError {
    pub fn new(msg: &str, status_code: StatusCode) -> AppError {
        AppError {
            details: msg.to_string(),
            status_code,
        }
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for AppError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl From<S3Error> for AppError {
    fn from(err: S3Error) -> AppError {
        AppError::new(&err.to_string(), StatusCode::INTERNAL_SERVER_ERROR)
    }
}

impl From<SdkError<GetObjectError>> for AppError {
    fn from(err: SdkError<GetObjectError>) -> AppError {
        AppError::new(&err.to_string(), StatusCode::INTERNAL_SERVER_ERROR)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (self.status_code, self.details).into_response()
    }
}

impl From<String> for AppError {
    fn from(err: String) -> AppError {
        AppError::new(&err, StatusCode::INTERNAL_SERVER_ERROR)
    }
}
