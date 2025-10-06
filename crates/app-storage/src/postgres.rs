use crate::Result;
use sqlx::PgPool;

/// PostgreSQL repository
pub struct PostgresRepository {
    pool: PgPool,
}

impl PostgresRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Get database pool
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    /// Health check
    pub async fn health_check(&self) -> Result<()> {
        sqlx::query("SELECT 1")
            .execute(&self.pool)
            .await
            .map_err(|e| crate::StorageError::DatabaseError(e.to_string()))?;
        Ok(())
    }
}
