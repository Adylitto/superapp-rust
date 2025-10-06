use axum::{
    response::{Json, IntoResponse, Response},
    http::StatusCode,
    extract::rejection::JsonRejection,
};
use serde_json::json;
use std::convert::Infallible;

// Define application-specific error types
#[derive(Debug)]
pub enum AppError {
    ValidationError(String),
    DatabaseError(String),
    AuthError(String),
    InternalError(String),
}

impl AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::ValidationError(_) => StatusCode::BAD_REQUEST,
            AppError::AuthError(_) => StatusCode::UNAUTHORIZED,
            AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn message(&self) -> String {
        match self {
            AppError::ValidationError(msg) => msg.clone(),
            AppError::DatabaseError(msg) => format!("Database error: {}", msg),
            AppError::AuthError(msg) => format!("Authentication error: {}", msg),
            AppError::InternalError(msg) => format!("Internal error: {}", msg),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let body = Json(json!({
            "error": self.message(),
            "status_code": status.as_u16(),
        }));

        (status, body).into_response()
    }
}

impl<E: std::error::Error> From<E> for AppError {
    fn from(err: E) -> Self {
        AppError::InternalError(err.to_string())
    }
}

// Custom result type for our application
pub type AppResult<T> = Result<T, AppError>;

// Handler for JSON parsing errors
pub async fn handle_json_error(rejection: JsonRejection) -> impl IntoResponse {
    let status = rejection.status();
    let error_msg = rejection.body_text();
    
    (
        status,
        Json(json!({
            "error": format!("Invalid JSON: {}", error_msg),
            "status_code": status.as_u16(),
        }))
    )
}

// Handler for any other errors (should not happen with our error handling)
pub async fn handle_other_error(err: Infallible) -> impl IntoResponse {
    match err {}
}