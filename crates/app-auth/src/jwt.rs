use crate::{AuthError, Result};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // user_id
    pub exp: usize,   // expiration time
    pub iat: usize,   // issued at
}

/// JWT token service
pub struct JwtService {
    secret: String,
}

impl JwtService {
    pub fn new(secret: String) -> Self {
        Self { secret }
    }

    /// Generate JWT token
    pub fn generate_token(&self, user_id: Uuid) -> Result<String> {
        let now = chrono::Utc::now().timestamp() as usize;
        let claims = Claims {
            sub: user_id.to_string(),
            exp: now + 3600, // 1 hour
            iat: now,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|e| AuthError::Internal(e.into()))
    }

    /// Validate JWT token
    pub fn validate_token(&self, token: &str) -> Result<Uuid> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| AuthError::InvalidToken)?;

        Uuid::parse_str(&token_data.claims.sub)
            .map_err(|e| AuthError::Internal(e.into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jwt_generation_and_validation() {
        let service = JwtService::new("test_secret".to_string());
        let user_id = Uuid::new_v4();

        let token = service.generate_token(user_id).unwrap();
        let validated_id = service.validate_token(&token).unwrap();

        assert_eq!(user_id, validated_id);
    }
}
