// Storage module
// Handles database operations, caching, and data persistence

pub mod postgres;
pub mod redis_cache;
pub mod repositories;

pub use postgres::PostgresRepository;
pub use redis_cache::RedisCache;
pub use repositories::{user_repository::UserRepositoryImpl, post_repository::PostRepositoryImpl};

/// Storage module errors
#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Cache error: {0}")]
    CacheError(String),

    #[error("Not found")]
    NotFound,

    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, StorageError>;