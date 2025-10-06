use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateProposalRequest {
    pub title: String,
    pub description: String,
    pub proposal_type: String,
    pub voting_duration_hours: i64,
}

#[derive(Debug, Serialize)]
pub struct CreateProposalResponse {
    pub proposal_id: String,
    pub status: String,
    pub voting_ends_at: String,
}

/// Create DAO proposal endpoint
pub async fn create_proposal(
    Json(payload): Json<CreateProposalRequest>,
) -> impl IntoResponse {
    // TODO: Implement actual proposal creation with blockchain integration

    tracing::info!("Creating proposal: {}", payload.title);

    let voting_ends_at = chrono::Utc::now()
        + chrono::Duration::hours(payload.voting_duration_hours);

    let response = CreateProposalResponse {
        proposal_id: uuid::Uuid::new_v4().to_string(),
        status: "draft".to_string(),
        voting_ends_at: voting_ends_at.to_rfc3339(),
    };

    (StatusCode::CREATED, Json(response))
}
