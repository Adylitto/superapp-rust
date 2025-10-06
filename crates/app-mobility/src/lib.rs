// Mobility/ride-sharing module
// Handles ride requests, matching, tracking, and optimization

pub mod service;

pub use service::MobilityService;

/// Mobility module errors
#[derive(Debug, thiserror::Error)]
pub enum MobilityError {
    #[error("Ride not found: {0}")]
    RideNotFound(String),

    #[error("No drivers available")]
    NoDriversAvailable,

    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, MobilityError>;
