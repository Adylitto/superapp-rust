use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Ride entity for mobility services
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ride {
    pub id: Uuid,
    pub rider_id: Uuid,
    pub driver_id: Option<Uuid>,
    pub origin: Location,
    pub destination: Location,
    pub status: RideStatus,
    pub pricing: RidePricing,
    pub estimated_duration: u32, // minutes
    pub actual_duration: Option<u32>,
    pub ai_optimized_route: Option<Vec<Location>>,
    pub tokens_earned: u64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
    pub address: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum RideStatus {
    Requested,
    Matched,
    DriverEnRoute,
    InProgress,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RidePricing {
    pub base_fare: f64,
    pub distance_fare: f64,
    pub time_fare: f64,
    pub total: f64,
    pub currency: String,
}

impl Ride {
    /// Create a new ride request
    pub fn new(
        rider_id: Uuid,
        origin: Location,
        destination: Location,
        pricing: RidePricing,
        estimated_duration: u32,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            rider_id,
            driver_id: None,
            origin,
            destination,
            status: RideStatus::Requested,
            pricing,
            estimated_duration,
            actual_duration: None,
            ai_optimized_route: None,
            tokens_earned: 0,
            created_at: now,
            updated_at: now,
        }
    }

    /// Match ride with a driver
    pub fn match_driver(&mut self, driver_id: Uuid) -> Result<(), crate::Error> {
        if self.status != RideStatus::Requested {
            return Err(crate::Error::BusinessRule(
                "Ride must be in Requested status to match driver".to_string(),
            ));
        }
        self.driver_id = Some(driver_id);
        self.status = RideStatus::Matched;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Start the ride
    pub fn start(&mut self) -> Result<(), crate::Error> {
        if self.status != RideStatus::DriverEnRoute {
            return Err(crate::Error::BusinessRule(
                "Driver must be en route to start ride".to_string(),
            ));
        }
        self.status = RideStatus::InProgress;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Complete the ride and calculate token reward
    /// 10 tokens per completed ride
    pub fn complete(&mut self, actual_duration: u32) -> Result<(), crate::Error> {
        if self.status != RideStatus::InProgress {
            return Err(crate::Error::BusinessRule(
                "Ride must be in progress to complete".to_string(),
            ));
        }
        self.status = RideStatus::Completed;
        self.actual_duration = Some(actual_duration);
        self.tokens_earned = 10; // Fixed reward per ride
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Set AI-optimized route
    pub fn set_ai_route(&mut self, route: Vec<Location>) {
        self.ai_optimized_route = Some(route);
        self.updated_at = Utc::now();
    }

    /// Calculate token reward for ride completion
    pub fn calculate_token_reward(&self) -> u64 {
        if self.status == RideStatus::Completed {
            10
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ride_lifecycle() {
        let rider_id = Uuid::new_v4();
        let driver_id = Uuid::new_v4();

        let origin = Location {
            latitude: 37.7749,
            longitude: -122.4194,
            address: Some("San Francisco".to_string()),
        };

        let destination = Location {
            latitude: 37.8044,
            longitude: -122.2712,
            address: Some("Oakland".to_string()),
        };

        let pricing = RidePricing {
            base_fare: 5.0,
            distance_fare: 10.0,
            time_fare: 5.0,
            total: 20.0,
            currency: "USD".to_string(),
        };

        let mut ride = Ride::new(rider_id, origin, destination, pricing, 30);

        assert_eq!(ride.status, RideStatus::Requested);
        assert!(ride.match_driver(driver_id).is_ok());
        assert_eq!(ride.status, RideStatus::Matched);

        ride.status = RideStatus::DriverEnRoute;
        assert!(ride.start().is_ok());
        assert_eq!(ride.status, RideStatus::InProgress);

        assert!(ride.complete(25).is_ok());
        assert_eq!(ride.status, RideStatus::Completed);
        assert_eq!(ride.tokens_earned, 10);
    }
}
