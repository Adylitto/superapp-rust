// Real-time messaging module
// Handles WebSocket connections, chat, and E2E encryption

pub mod service;

pub use service::MessagingService;

/// Messaging module errors
#[derive(Debug, thiserror::Error)]
pub enum MessagingError {
    #[error("Message not found: {0}")]
    MessageNotFound(String),

    #[error("Connection failed: {0}")]
    ConnectionFailed(String),

    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, MessagingError>;
