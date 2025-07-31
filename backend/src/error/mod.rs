use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Validation failed: {0}")]
    Validation(String),
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Internal server error: {0}")]
    Internal(String)
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
    code: u16
}

impl ResponseError for AppError {
    fn error_response(&self) -> actix_web::HttpResponse {
        let (status, code) = match self {
            AppError::Validation(_) => (StatusCode::BAD_REQUEST, 400),
            AppError::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, 500),
            AppError::NotFound(_) => (StatusCode::NOT_FOUND, 404),
            AppError::Unauthorized(_) => (StatusCode::UNAUTHORIZED, 401)
        };

        let err_msg = self.to_string();

        HttpResponse::build(status).json(
            ErrorResponse{
                error: err_msg,
                code
            }
        )
    }
}