use crate::{Error, Result, TokenTransaction, TransactionReason};
use super::user_repository::UserRepository;
use uuid::Uuid;

/// Use case for granting tokens to users based on various actions
pub struct GrantTokensUseCase {
    user_repository: Box<dyn UserRepository>,
    transaction_repository: Box<dyn TransactionRepository>,
}

/// Repository trait for transaction operations
#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait TransactionRepository: Send + Sync {
    async fn create(&self, transaction: &TokenTransaction) -> Result<()>;
    async fn confirm(&self, transaction_id: Uuid, tx_hash: String) -> Result<()>;
}

impl GrantTokensUseCase {
    pub fn new(
        user_repository: Box<dyn UserRepository>,
        transaction_repository: Box<dyn TransactionRepository>,
    ) -> Self {
        Self {
            user_repository,
            transaction_repository,
        }
    }

    /// Grant tokens for post engagement
    pub async fn grant_for_post_engagement(
        &self,
        user_id: Uuid,
        post_likes: u32,
    ) -> Result<TokenTransaction> {
        // Business rule: 5 tokens if post has >= 10 likes
        if post_likes < 10 {
            return Err(Error::BusinessRule(
                "Post must have at least 10 likes to earn tokens".to_string(),
            ));
        }

        self.grant_tokens(user_id, 5, TransactionReason::PostEngagement)
            .await
    }

    /// Grant tokens for ride completion
    pub async fn grant_for_ride_completion(
        &self,
        user_id: Uuid,
    ) -> Result<TokenTransaction> {
        // Business rule: 10 tokens per completed ride
        self.grant_tokens(user_id, 10, TransactionReason::RideCompletion)
            .await
    }

    /// Grant tokens for referral
    pub async fn grant_for_referral(&self, user_id: Uuid) -> Result<TokenTransaction> {
        // Business rule: 20 tokens per successful referral
        self.grant_tokens(user_id, 20, TransactionReason::Referral)
            .await
    }

    /// Core token granting logic
    async fn grant_tokens(
        &self,
        user_id: Uuid,
        amount: u64,
        reason: TransactionReason,
    ) -> Result<TokenTransaction> {
        // Find user
        let mut user = self
            .user_repository
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| Error::NotFound(format!("User {} not found", user_id)))?;

        // Add tokens to user balance
        user.add_tokens(amount);

        // Create transaction record
        let transaction = TokenTransaction::create_reward(user_id, amount, reason);

        // Persist changes
        self.transaction_repository.create(&transaction).await?;
        self.user_repository.update(&user).await?;

        // In production, this would interact with blockchain to mint tokens
        // For now, we simulate by confirming the transaction
        self.transaction_repository
            .confirm(transaction.id, format!("0xsimulated_{}", transaction.id))
            .await?;

        Ok(transaction)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::user_repository::MockUserRepository;
    use crate::User;
    use mockall::predicate::*;

    #[tokio::test]
    async fn test_grant_for_post_engagement_success() {
        let user_id = Uuid::new_v4();
        let mut user = User::new(
            "test@example.com".to_string(),
            "testuser".to_string(),
            "hash".to_string(),
        );
        user.id = user_id;

        let mut user_repo = Box::new(MockUserRepository::new());
        user_repo
            .expect_find_by_id()
            .with(eq(user_id))
            .returning(move |_| Ok(Some(user.clone())));
        user_repo.expect_update().returning(|_| Ok(()));

        let mut tx_repo = Box::new(MockTransactionRepository::new());
        tx_repo.expect_create().returning(|_| Ok(()));
        tx_repo.expect_confirm().returning(|_, _| Ok(()));

        let use_case = GrantTokensUseCase::new(user_repo, tx_repo);
        let result = use_case.grant_for_post_engagement(user_id, 15).await;

        assert!(result.is_ok());
        let tx = result.unwrap();
        assert_eq!(tx.amount, 5);
    }

    #[tokio::test]
    async fn test_grant_for_post_engagement_insufficient_likes() {
        let user_id = Uuid::new_v4();
        let user_repo = Box::new(MockUserRepository::new());
        let tx_repo = Box::new(MockTransactionRepository::new());

        let use_case = GrantTokensUseCase::new(user_repo, tx_repo);
        let result = use_case.grant_for_post_engagement(user_id, 5).await;

        assert!(result.is_err());
    }
}
