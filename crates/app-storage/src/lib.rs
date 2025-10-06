// Storage module
// Handles database operations, caching, and data persistence

pub mod postgres;
pub mod redis_cache;

pub use postgres::PostgresRepository;
pub use redis_cache::RedisCache;

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
