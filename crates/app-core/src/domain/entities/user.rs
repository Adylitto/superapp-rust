use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// User entity - core identity in the system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub password_hash: String,
    pub profile: UserProfile,
    pub token_balance: u64,
    pub reputation_score: f64,
    pub is_verified: bool,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub full_name: String,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub location: Option<String>,
    pub phone: Option<String>,
    pub preferences: UserPreferences,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserPreferences {
    pub language: String,
    pub notifications_enabled: bool,
    pub ai_recommendations_enabled: bool,
    pub location_sharing_enabled: bool,
}

impl User {
    /// Create a new user with default values
    pub fn new(email: String, username: String, password_hash: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            email,
            username,
            password_hash,
            profile: UserProfile {
                full_name: String::new(),
                avatar_url: None,
                bio: None,
                location: None,
                phone: None,
                preferences: UserPreferences {
                    language: "en".to_string(),
                    ..Default::default()
                },
            },
            token_balance: 0,
            reputation_score: 0.0,
            is_verified: false,
            is_active: true,
            created_at: now,
            updated_at: now,
        }
    }

    /// Check if user has sufficient tokens
    pub fn has_sufficient_tokens(&self, amount: u64) -> bool {
        self.token_balance >= amount
    }

    /// Add tokens to user balance
    pub fn add_tokens(&mut self, amount: u64) {
        self.token_balance += amount;
        self.updated_at = Utc::now();
    }

    /// Deduct tokens from user balance
    pub fn deduct_tokens(&mut self, amount: u64) -> Result<(), crate::Error> {
        if !self.has_sufficient_tokens(amount) {
            return Err(crate::Error::InsufficientTokens {
                required: amount,
                available: self.token_balance,
            });
        }
        self.token_balance -= amount;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Update reputation score
    pub fn update_reputation(&mut self, delta: f64) {
        self.reputation_score = (self.reputation_score + delta).max(0.0).min(100.0);
        self.updated_at = Utc::now();
    }

    /// Verify user account
    pub fn verify(&mut self) {
        self.is_verified = true;
        self.updated_at = Utc::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation() {
        let user = User::new(
            "test@example.com".to_string(),
            "testuser".to_string(),
            "hashed_password".to_string(),
        );
        assert_eq!(user.email, "test@example.com");
        assert_eq!(user.token_balance, 0);
        assert!(!user.is_verified);
    }

    #[test]
    fn test_token_operations() {
        let mut user = User::new(
            "test@example.com".to_string(),
            "testuser".to_string(),
            "hash".to_string(),
        );

        user.add_tokens(100);
        assert_eq!(user.token_balance, 100);

        assert!(user.deduct_tokens(50).is_ok());
        assert_eq!(user.token_balance, 50);

        assert!(user.deduct_tokens(100).is_err());
    }
}
