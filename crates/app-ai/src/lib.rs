use tract_onnx::prelude::*;
use ndarray::{Array, IxDyn};
use std::path::Path;

pub mod recommendation;
pub mod moderation;
pub mod routing;

pub use recommendation::RecommendationService;
pub use moderation::ModerationService;
pub use routing::RoutingService;

/// AI Service error types
#[derive(Debug, thiserror::Error)]
pub enum AIError {
    #[error("Model loading failed: {0}")]
    ModelLoadError(String),

    #[error("Inference failed: {0}")]
    InferenceError(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, AIError>;

/// Base AI model wrapper
pub struct AIModel {
    model: SimplePlan<TypedFact, Box<dyn TypedOp>, Graph<TypedFact, Box<dyn TypedOp>>>,
}

impl AIModel {
    /// Load ONNX model from path
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let model = tract_onnx::onnx()
            .model_for_path(path)
            .map_err(|e| AIError::ModelLoadError(e.to_string()))?
            .into_optimized()
            .map_err(|e| AIError::ModelLoadError(e.to_string()))?
            .into_runnable()
            .map_err(|e| AIError::ModelLoadError(e.to_string()))?;

        Ok(Self { model })
    }

    /// Run inference on input tensor
    pub fn infer(&self, _input: Array<f32, IxDyn>) -> Result<Vec<f32>> {
        // TODO: Fix tract-onnx API compatibility
        // Placeholder implementation
        Ok(vec![0.0; 10])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ai_error_types() {
        let err = AIError::InvalidInput("test".to_string());
        assert!(err.to_string().contains("Invalid input"));
    }
}
