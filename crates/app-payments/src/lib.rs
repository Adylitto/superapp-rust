// Payments module
// Handles fiat and crypto payments, wallets, and transactions

pub mod service;

pub use service::PaymentService;

/// Payment module errors
#[derive(Debug, thiserror::Error)]
pub enum PaymentError {
    #[error("Payment failed: {0}")]
    PaymentFailed(String),

    #[error("Insufficient balance")]
    InsufficientBalance,

    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, PaymentError>;
