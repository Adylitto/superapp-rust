
use crate::{Result, User};
use uuid::Uuid;

/// Repository trait for user operations
#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>>;
    async fn find_by_ids(&self, ids: Vec<Uuid>) -> Result<Vec<User>>;
    async fn update(&self, user: &User) -> Result<()>;
}
