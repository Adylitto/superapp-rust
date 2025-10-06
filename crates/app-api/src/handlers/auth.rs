use axum::{http::StatusCode, response::IntoResponse, extract::State, Json};
use serde::{Deserialize, Serialize};
use app_auth::AuthService;

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    pub user_id: String,
    pub token: String,
}

/// User registration endpoint
pub async fn register(
    State(auth_service): State<AuthService>,
    Json(payload): Json<RegisterRequest>,
) -> impl IntoResponse {
    match auth_service.register(payload.email, payload.username, payload.password).await {
        Ok((user_id, token)) => {
            let response = RegisterResponse {
                user_id: user_id.to_string(),
                token,
            };
            (StatusCode::CREATED, Json(response))
        }
        Err(e) => {
            tracing::error!("Registration failed: {}", e);
            (StatusCode::BAD_REQUEST, Json(serde_json::json!({
                "error": "Registration failed",
                "message": e.to_string()
            })))
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub refresh_token: String,
}

/// User login endpoint
pub async fn login(
    State(auth_service): State<AuthService>,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    match auth_service.login(payload.email, payload.password).await {
        Ok(token) => {
            let response = LoginResponse {
                token,
                refresh_token: "refresh_token_placeholder".to_string(), // In a real app, generate a refresh token
            };
            Json(response)
        }
        Err(e) => {
            tracing::error!("Login failed: {}", e);
            (StatusCode::UNAUTHORIZED, Json(serde_json::json!({
                "error": "Invalid credentials",
                "message": e.to_string()
            })))
        }
    }
}