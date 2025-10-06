// Social networking module
// Handles posts, feeds, friends, groups, and social interactions

pub mod service;

pub use service::SocialService;

/// Social module errors
#[derive(Debug, thiserror::Error)]
pub enum SocialError {
    #[error("Post not found: {0}")]
    PostNotFound(String),

    #[error("User not found: {0}")]
    UserNotFound(String),

    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, SocialError>;
