use crate::{JwtService, PasswordService, Result};
use app_storage::repositories::user_repository::UserRepositoryImpl;
use uuid::Uuid;

/// Authentication service
pub struct AuthService {
    jwt: JwtService,
    password: PasswordService,
    user_repo: UserRepositoryImpl,
}

impl AuthService {
    pub fn new(jwt_secret: String, user_repo: UserRepositoryImpl) -> Self {
        Self {
            jwt: JwtService::new(jwt_secret),
            password: PasswordService::new(),
            user_repo,
        }
    }

    /// Register new user
    pub async fn register(
        &self,
        email: String,
        username: String,
        password: String,
    ) -> Result<(Uuid, String)> {
        // Check if user already exists
        if self.user_repo.find_by_email(&email).await?.is_some() {
            return Err(crate::AuthError::Internal(anyhow::anyhow!("Email already exists")));
        }
        
        if self.user_repo.find_by_username(&username).await?.is_some() {
            return Err(crate::AuthError::Internal(anyhow::anyhow!("Username already exists")));
        }

        // Hash password
        let password_hash = self.password.hash(&password)?;

        // Create new user
        let user = app_core::User::new(email, username, password_hash);
        let user_id = user.id;

        // Store user in database
        self.user_repo.create(&user).await.map_err(|e| {
            crate::AuthError::Internal(anyhow::anyhow!("Failed to create user: {}", e))
        })?;

        // Generate JWT token
        let token = self.jwt.generate_token(user_id)?;
        Ok((user_id, token))
    }

    /// Login user
    pub async fn login(&self, email: String, password: String) -> Result<String> {
        // Fetch user from database
        let user = self.user_repo.find_by_email(&email).await?
            .ok_or(crate::AuthError::InvalidCredentials)?;

        // Verify password
        if !self.password.verify(&password, &user.password_hash)? {
            return Err(crate::AuthError::InvalidCredentials);
        }

        // Generate JWT token
        let token = self.jwt.generate_token(user.id)?;
        Ok(token)
    }

    /// Validate token
    pub fn validate_token(&self, token: &str) -> Result<Uuid> {
        self.jwt.validate_token(token)
    }
}