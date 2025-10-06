// Domain events for event-driven architecture
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DomainEvent {
    UserRegistered(UserRegisteredEvent),
    TokensGranted(TokensGrantedEvent),
    ProposalCreated(ProposalCreatedEvent),
    RideCompleted(RideCompletedEvent),
    MessageSent(MessageSentEvent),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRegisteredEvent {
    pub user_id: Uuid,
    pub email: String,
    pub occurred_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokensGrantedEvent {
    pub user_id: Uuid,
    pub amount: u64,
    pub reason: String,
    pub occurred_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposalCreatedEvent {
    pub proposal_id: Uuid,
    pub proposer_id: Uuid,
    pub title: String,
    pub occurred_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RideCompletedEvent {
    pub ride_id: Uuid,
    pub rider_id: Uuid,
    pub driver_id: Uuid,
    pub tokens_earned: u64,
    pub occurred_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageSentEvent {
    pub message_id: Uuid,
    pub sender_id: Uuid,
    pub recipient_id: Uuid,
    pub occurred_at: DateTime<Utc>,
}
