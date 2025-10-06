use crate::{JwtService, PasswordService, Result};
use uuid::Uuid;

/// Authentication service
pub struct AuthService {
    jwt: JwtService,
    password: PasswordService,
}

impl AuthService {
    pub fn new(jwt_secret: String) -> Self {
        Self {
            jwt: JwtService::new(jwt_secret),
            password: PasswordService::new(),
        }
    }

    /// Register new user
    pub async fn register(
        &self,
        email: String,
        password: String,
    ) -> Result<(Uuid, String)> {
        let user_id = Uuid::new_v4();
        let _password_hash = self.password.hash(&password)?;

        // TODO: Store user in database

        let token = self.jwt.generate_token(user_id)?;
        Ok((user_id, token))
    }

    /// Login user
    pub async fn login(&self, email: String, password: String) -> Result<String> {
        // TODO: Fetch user from database and verify password
        let user_id = Uuid::new_v4();
        let token = self.jwt.generate_token(user_id)?;
        Ok(token)
    }

    /// Validate token
    pub fn validate_token(&self, token: &str) -> Result<Uuid> {
        self.jwt.validate_token(token)
    }
}
