use crate::Result;
use redis::{aio::ConnectionManager, AsyncCommands};

/// Redis cache service
pub struct RedisCache {
    client: ConnectionManager,
}

impl RedisCache {
    pub async fn new(url: &str) -> Result<Self> {
        let client = redis::Client::open(url)
            .map_err(|e| crate::StorageError::CacheError(e.to_string()))?;

        let manager = ConnectionManager::new(client)
            .await
            .map_err(|e| crate::StorageError::CacheError(e.to_string()))?;

        Ok(Self { client: manager })
    }

    /// Set value with expiration
    pub async fn set(&mut self, key: &str, value: &str, ttl_seconds: u64) -> Result<()> {
        self.client
            .set_ex(key, value, ttl_seconds)
            .await
            .map_err(|e| crate::StorageError::CacheError(e.to_string()))?;
        Ok(())
    }

    /// Get value
    pub async fn get(&mut self, key: &str) -> Result<Option<String>> {
        let value: Option<String> = self
            .client
            .get(key)
            .await
            .map_err(|e| crate::StorageError::CacheError(e.to_string()))?;
        Ok(value)
    }

    /// Delete value
    pub async fn delete(&mut self, key: &str) -> Result<()> {
        self.client
            .del(key)
            .await
            .map_err(|e| crate::StorageError::CacheError(e.to_string()))?;
        Ok(())
    }
}
