use std::{error, fmt};

use axum::{http, response, Json};

use serde::Serialize;
use serde_json::json;

#[derive(Debug, Serialize)]
struct ErrorMessage {
    pub error: &'static str,
}

#[derive(Debug)]
pub struct AppError{
    pub status_code: http::StatusCode,
    message: ErrorMessage,
}

impl response::IntoResponse for AppError {
    fn into_response(self) -> response::Response {
        (self.status_code, Json(json!(self.message))).into_response()
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message.error)
    }
}

impl From<anyhow::Error> for AppError {
    fn from(_: anyhow::Error) -> Self {
        AppError { 
            status_code: http::StatusCode::INTERNAL_SERVER_ERROR,
            message: ErrorMessage {
                error: "something went wrong"
            }
        }
    }
}

impl error::Error for AppError {} 

impl AppError {
    /// returns new instance of application error.
    pub fn new(status_code: http::StatusCode, message: &'static str) -> Self {
        AppError{
            status_code,
            message: ErrorMessage { error: message },
        }
    }
}


pub const SPENDING_NOT_FOUND_ERROR: AppError = AppError{
        status_code: http::StatusCode::NOT_FOUND,
        message: ErrorMessage { error: "spending not found" }
};


pub const INTERNAL_SERVER_ERROR: AppError = AppError{
    status_code: http::StatusCode::INTERNAL_SERVER_ERROR,
    message: ErrorMessage { error: "internal error" }
};
