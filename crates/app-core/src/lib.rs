// Core domain module - entities, value objects, and domain logic
pub mod domain;
pub mod use_cases;

// Re-exports for convenience
pub use domain::entities::*;
pub use domain::value_objects::*;
pub use domain::events::*;
pub use use_cases::*;

/// Result type for core domain operations
pub type Result<T> = std::result::Result<T, Error>;

/// Core domain error types
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Entity not found: {0}")]
    NotFound(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Authorization error: {0}")]
    Unauthorized(String),

    #[error("Business rule violation: {0}")]
    BusinessRule(String),

    #[error("Insufficient tokens: required {required}, available {available}")]
    InsufficientTokens { required: u64, available: u64 },

    #[error("External service error: {0}")]
    ExternalService(String),

    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}
