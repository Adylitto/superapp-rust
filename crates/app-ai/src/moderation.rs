use crate::{AIModel, Result};
use serde::{Deserialize, Serialize};

/// AI-powered content moderation service
pub struct ModerationService {
    model: Option<AIModel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModerationResult {
    pub is_safe: bool,
    pub confidence: f64,
    pub categories: Vec<ModerationCategory>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModerationCategory {
    pub name: String,
    pub score: f64,
}

impl ModerationService {
    /// Create new moderation service
    pub fn new() -> Self {
        Self { model: None }
    }

    /// Load moderation model
    pub fn load_model(&mut self, model_path: &str) -> Result<()> {
        let model = AIModel::load(model_path)?;
        self.model = Some(model);
        Ok(())
    }

    /// Moderate text content
    /// Detects: hate speech, violence, sexual content, spam
    pub async fn moderate_text(&self, content: &str) -> Result<ModerationResult> {
        // TODO: In production, this would:
        // 1. Tokenize text
        // 2. Run through transformer model (e.g., BERT-based classifier)
        // 3. Return category scores
        // 4. Apply threshold for safety decision

        // Simple rule-based mock for now
        let is_safe = !self.contains_unsafe_content(content);
        let confidence = if is_safe { 0.95 } else { 0.85 };

        Ok(ModerationResult {
            is_safe,
            confidence,
            categories: vec![
                ModerationCategory {
                    name: "hate_speech".to_string(),
                    score: 0.02,
                },
                ModerationCategory {
                    name: "violence".to_string(),
                    score: 0.01,
                },
                ModerationCategory {
                    name: "sexual".to_string(),
                    score: 0.03,
                },
            ],
        })
    }

    /// Moderate image content
    pub async fn moderate_image(&self, image_url: &str) -> Result<ModerationResult> {
        // TODO: Implement image moderation using vision models
        Ok(ModerationResult {
            is_safe: true,
            confidence: 0.90,
            categories: vec![],
        })
    }

    /// Simple rule-based content check
    fn contains_unsafe_content(&self, content: &str) -> bool {
        let unsafe_keywords = ["badword1", "badword2", "spam"];
        let content_lower = content.to_lowercase();

        unsafe_keywords
            .iter()
            .any(|keyword| content_lower.contains(keyword))
    }
}

impl Default for ModerationService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_moderate_safe_content() {
        let service = ModerationService::new();
        let result = service
            .moderate_text("Hello, this is a nice message!")
            .await
            .unwrap();

        assert!(result.is_safe);
        assert!(result.confidence > 0.8);
    }

    #[tokio::test]
    async fn test_moderate_unsafe_content() {
        let service = ModerationService::new();
        let result = service.moderate_text("spam spam spam").await.unwrap();

        assert!(!result.is_safe);
    }
}
