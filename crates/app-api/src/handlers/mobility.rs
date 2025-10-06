use axum::{http::StatusCode, response::IntoResponse, extract::State, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct RequestRideRequest {
    pub origin: LocationRequest,
    pub destination: LocationRequest,
}

#[derive(Debug, Deserialize)]
pub struct LocationRequest {
    pub latitude: f64,
    pub longitude: f64,
    pub address: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct RequestRideResponse {
    pub ride_id: String,
    pub status: String,
    pub estimated_duration: u32,
    pub estimated_cost: f64,
}

/// Request ride endpoint
pub async fn request_ride(
    // State(mobility_service): State<MobilityService>,  // We'll implement this later
    Json(payload): Json<RequestRideRequest>,
) -> impl IntoResponse {
    // TODO: Implement actual ride matching with AI route optimization
    tracing::info!(
        "Ride requested from ({}, {}) to ({}, {})",
        payload.origin.latitude,
        payload.origin.longitude,
        payload.destination.latitude,
        payload.destination.longitude
    );

    let response = RequestRideResponse {
        ride_id: uuid::Uuid::new_v4().to_string(),
        status: "requested".to_string(),
        estimated_duration: 25,
        estimated_cost: 18.50,
    };

    (StatusCode::CREATED, Json(response))
}