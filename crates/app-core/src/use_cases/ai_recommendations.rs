use crate::{Error, Result, User};
use super::user_repository::UserRepository;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Use case for AI-powered recommendations
pub struct AIRecommendationsUseCase {
    ai_service: Box<dyn AIService>,
    user_repository: Box<dyn UserRepository>,
}

/// AI service trait for machine learning operations
#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait AIService: Send + Sync {
    async fn recommend_connections(&self, user_id: Uuid, limit: usize) -> Result<Vec<UserRecommendation>>;
    async fn optimize_route(&self, origin: (f64, f64), destination: (f64, f64)) -> Result<RouteOptimization>;
    async fn moderate_content(&self, content: &str) -> Result<ContentModerationResult>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRecommendation {
    pub user_id: Uuid,
    pub score: f64,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteOptimization {
    pub waypoints: Vec<(f64, f64)>,
    pub estimated_time: u32,
    pub estimated_distance: f64,
    pub efficiency_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentModerationResult {
    pub is_safe: bool,
    pub confidence: f64,
    pub categories: Vec<String>,
}

impl AIRecommendationsUseCase {
    pub fn new(
        ai_service: Box<dyn AIService>,
        user_repository: Box<dyn UserRepository>,
    ) -> Self {
        Self {
            ai_service,
            user_repository,
        }
    }

    /// Get AI-powered friend/connection recommendations
    pub async fn get_connection_recommendations(
        &self,
        user_id: Uuid,
        limit: usize,
    ) -> Result<Vec<User>> {
        // Verify user exists
        let _user = self
            .user_repository
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| Error::NotFound(format!("User {} not found", user_id)))?;

        // Check if AI recommendations are enabled
        if !_user.profile.preferences.ai_recommendations_enabled {
            return Err(Error::BusinessRule(
                "AI recommendations are disabled for this user".to_string(),
            ));
        }

        // Get AI recommendations
        let recommendations = self
            .ai_service
            .recommend_connections(user_id, limit)
            .await?;

        // Fetch full user profiles
        let user_ids: Vec<Uuid> = recommendations.iter().map(|r| r.user_id).collect();
        let users = self.user_repository.find_by_ids(user_ids).await?;

        Ok(users)
    }

    /// Get AI-optimized route for ride
    pub async fn get_optimized_route(
        &self,
        origin: (f64, f64),
        destination: (f64, f64),
    ) -> Result<RouteOptimization> {
        self.ai_service.optimize_route(origin, destination).await
    }

    /// Moderate content using AI
    pub async fn moderate_content(&self, content: &str) -> Result<ContentModerationResult> {
        if content.is_empty() {
            return Err(Error::Validation("Content cannot be empty".to_string()));
        }

        self.ai_service.moderate_content(content).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::user_repository::MockUserRepository;
    use crate::User;

    #[tokio::test]
    async fn test_get_connection_recommendations() {
        let user_id = Uuid::new_v4();
        let mut user = User::new(
            "test@example.com".to_string(),
            "testuser".to_string(),
            "hash".to_string(),
        );
        user.id = user_id;
        user.profile.preferences.ai_recommendations_enabled = true;

        let recommended_id = Uuid::new_v4();
        let recommended_user = User::new(
            "friend@example.com".to_string(),
            "friend".to_string(),
            "hash".to_string(),
        );

        let mut ai_service = Box::new(MockAIService::new());
        ai_service
            .expect_recommend_connections()
            .returning(move |_, _| {
                Ok(vec![UserRecommendation {
                    user_id: recommended_id,
                    score: 0.85,
                    reason: "Similar interests".to_string(),
                }])
            });

        let mut user_repo = Box::new(MockUserRepository::new());
        user_repo
            .expect_find_by_id()
            .returning(move |_| Ok(Some(user.clone())));
        user_repo
            .expect_find_by_ids()
            .returning(move |_| Ok(vec![recommended_user.clone()]));

        let use_case = AIRecommendationsUseCase::new(ai_service, user_repo);
        let result = use_case.get_connection_recommendations(user_id, 5).await;

        assert!(result.is_ok());
        let recommendations = result.unwrap();
        assert_eq!(recommendations.len(), 1);
    }
}
