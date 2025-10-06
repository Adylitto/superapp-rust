use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Token transaction record for audit and accounting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenTransaction {
    pub id: Uuid,
    pub from_user_id: Option<Uuid>,
    pub to_user_id: Option<Uuid>,
    pub amount: u64,
    pub transaction_type: TransactionType,
    pub reason: TransactionReason,
    pub blockchain_tx_hash: Option<String>,
    pub status: TransactionStatus,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionType {
    Mint,      // Creating new tokens
    Burn,      // Destroying tokens
    Transfer,  // User to user
    Reward,    // System reward
    Stake,     // Locking tokens
    Unstake,   // Unlocking tokens
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionReason {
    PostEngagement,
    RideCompletion,
    Referral,
    DailyBonus,
    GovernanceVote,
    MiniAppUsage,
    Payment,
    Custom(String),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum TransactionStatus {
    Pending,
    Confirmed,
    Failed,
    Reversed,
}

impl TokenTransaction {
    /// Create a new token transaction
    pub fn new(
        from_user_id: Option<Uuid>,
        to_user_id: Option<Uuid>,
        amount: u64,
        transaction_type: TransactionType,
        reason: TransactionReason,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            from_user_id,
            to_user_id,
            amount,
            transaction_type,
            reason,
            blockchain_tx_hash: None,
            status: TransactionStatus::Pending,
            metadata: serde_json::json!({}),
            created_at: Utc::now(),
        }
    }

    /// Create a reward transaction
    pub fn create_reward(user_id: Uuid, amount: u64, reason: TransactionReason) -> Self {
        Self::new(
            None,
            Some(user_id),
            amount,
            TransactionType::Reward,
            reason,
        )
    }

    /// Mark transaction as confirmed with blockchain hash
    pub fn confirm(&mut self, tx_hash: String) {
        self.status = TransactionStatus::Confirmed;
        self.blockchain_tx_hash = Some(tx_hash);
    }

    /// Mark transaction as failed
    pub fn fail(&mut self) {
        self.status = TransactionStatus::Failed;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reward_transaction() {
        let user_id = Uuid::new_v4();
        let mut tx = TokenTransaction::create_reward(
            user_id,
            10,
            TransactionReason::RideCompletion,
        );

        assert_eq!(tx.amount, 10);
        assert_eq!(tx.status, TransactionStatus::Pending);

        tx.confirm("0x123abc".to_string());
        assert_eq!(tx.status, TransactionStatus::Confirmed);
        assert!(tx.blockchain_tx_hash.is_some());
    }
}
