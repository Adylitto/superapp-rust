/// Supabase integration module
/// This module provides utilities for integrating with Supabase services
/// while maintaining our custom application logic
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SupabaseConfig {
    pub url: String,
    pub anon_key: String,
    pub service_role_key: String,
    pub jwt_secret: String,
}

impl SupabaseConfig {
    pub fn from_env() -> Option<Self> {
        // Only initialize if Supabase environment variables are present
        if let (Ok(url), Ok(anon_key), Ok(jwt_secret)) = (
            std::env::var("SUPABASE_URL"),
            std::env::var("SUPABASE_ANON_KEY"),
            std::env::var("SUPABASE_JWT_SECRET"),
        ) {
            let service_role_key = std::env::var("SUPABASE_SERVICE_ROLE_KEY").unwrap_or_default();
            
            Some(Self {
                url,
                anon_key,
                service_role_key,
                jwt_secret,
            })
        } else {
            None
        }
    }
}

/// Supabase Auth API client
pub struct SupabaseAuthClient {
    client: reqwest::Client,
    config: SupabaseConfig,
}

impl SupabaseAuthClient {
    pub fn new(config: SupabaseConfig) -> Self {
        Self {
            client: reqwest::Client::new(),
            config,
        }
    }

    /// Verify a JWT token from Supabase Auth
    pub async fn verify_token(&self, token: &str) -> Result<SupabaseUser, AuthError> {
        // In a real implementation, we'd call Supabase's JWT verification
        // For now, we'll use our existing JWT service but configured with Supabase's secret
        let validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256);
        let token_data = jsonwebtoken::decode::<SupabaseClaims>(
            token,
            &jsonwebtoken::DecodingKey::from_secret(self.config.jwt_secret.as_bytes()),
            &validation,
        )
        .map_err(|e| AuthError::InvalidToken)?;
        
        Ok(SupabaseUser {
            id: token_data.claims.sub,
            email: token_data.claims.email,
            // Additional claims can be added here
        })
    }
}

#[derive(Debug, Deserialize)]
struct SupabaseClaims {
    sub: String,      // User ID
    email: String,    // User email
    #[serde(rename = "exp")]
    expires_at: usize,
}

#[derive(Debug)]
pub struct SupabaseUser {
    pub id: String,
    pub email: String,
}

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("Invalid token")]
    InvalidToken,
    
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
    
    #[error("JWT error: {0}")]
    JwtError(#[from] jsonwebtoken::errors::Error),
    
    #[error("Internal error: {0}")]
    InternalError(String),
}

// For now, we'll use this to indicate Supabase integration is available
pub fn is_supabase_enabled() -> bool {
    std::env::var("SUPABASE_URL").is_ok()
        && std::env::var("SUPABASE_ANON_KEY").is_ok()
        && std::env::var("SUPABASE_JWT_SECRET").is_ok()
}