// Authentication and authorization module
// Handles JWT, password hashing, and access control

pub mod service;
pub mod jwt;
pub mod password;
pub mod supabase;

pub use service::AuthService;
pub use jwt::JwtService;
pub use password::PasswordService;
pub use supabase::{SupabaseAuthClient, SupabaseConfig, is_supabase_enabled};

/// Auth module errors
#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Token expired")]
    TokenExpired,

    #[error("Invalid token")]
    InvalidToken,

    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, AuthError>;
