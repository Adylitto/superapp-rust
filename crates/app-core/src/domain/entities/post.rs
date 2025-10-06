use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Social post entity with engagement tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: Uuid,
    pub author_id: Uuid,
    pub content: String,
    pub media_urls: Vec<String>,
    pub likes_count: u32,
    pub comments_count: u32,
    pub shares_count: u32,
    pub visibility: PostVisibility,
    pub is_ai_moderated: bool,
    pub moderation_score: f64,
    pub tokens_earned: u64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum PostVisibility {
    Public,
    Friends,
    Private,
}

impl Post {
    /// Create a new post
    pub fn new(author_id: Uuid, content: String, visibility: PostVisibility) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            author_id,
            content,
            media_urls: Vec::new(),
            likes_count: 0,
            comments_count: 0,
            shares_count: 0,
            visibility,
            is_ai_moderated: false,
            moderation_score: 0.0,
            tokens_earned: 0,
            created_at: now,
            updated_at: now,
        }
    }

    /// Add a like to the post
    pub fn add_like(&mut self) {
        self.likes_count += 1;
        self.updated_at = Utc::now();
    }

    /// Calculate tokens earned based on engagement
    /// 5 tokens per post if likes > 10
    pub fn calculate_token_reward(&self) -> u64 {
        if self.likes_count >= 10 {
            5
        } else {
            0
        }
    }

    /// Mark post as AI moderated
    pub fn set_moderation_result(&mut self, score: f64) {
        self.is_ai_moderated = true;
        self.moderation_score = score;
        self.updated_at = Utc::now();
    }

    /// Check if post is eligible for token reward
    pub fn is_reward_eligible(&self) -> bool {
        self.likes_count >= 10 && self.is_ai_moderated && self.moderation_score >= 0.8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_post_creation() {
        let author_id = Uuid::new_v4();
        let post = Post::new(
            author_id,
            "Test content".to_string(),
            PostVisibility::Public,
        );
        assert_eq!(post.author_id, author_id);
        assert_eq!(post.likes_count, 0);
    }

    #[test]
    fn test_token_reward_calculation() {
        let mut post = Post::new(
            Uuid::new_v4(),
            "Content".to_string(),
            PostVisibility::Public,
        );

        assert_eq!(post.calculate_token_reward(), 0);

        for _ in 0..10 {
            post.add_like();
        }
        assert_eq!(post.calculate_token_reward(), 5);
    }
}
