use crate::{AuthError, Result};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

/// Password hashing service using Argon2
pub struct PasswordService;

impl PasswordService {
    pub fn new() -> Self {
        Self
    }

    /// Hash password using Argon2
    pub fn hash(&self, password: &str) -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| AuthError::Internal(anyhow::anyhow!("Password hashing failed: {}", e)))?
            .to_string();

        Ok(password_hash)
    }

    /// Verify password against hash
    pub fn verify(&self, password: &str, hash: &str) -> Result<bool> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| AuthError::Internal(anyhow::anyhow!("Password parsing failed: {}", e)))?;

        let argon2 = Argon2::default();

        Ok(argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }
}

impl Default for PasswordService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hashing() {
        let service = PasswordService::new();
        let password = "securepassword123";

        let hash = service.hash(password).unwrap();
        assert!(service.verify(password, &hash).unwrap());
        assert!(!service.verify("wrongpassword", &hash).unwrap());
    }
}
