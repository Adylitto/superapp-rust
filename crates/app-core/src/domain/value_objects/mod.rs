// Value objects for domain modeling
// These are immutable types that represent concepts with no identity

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Email(String);

impl Email {
    pub fn new(email: String) -> Result<Self, String> {
        if email.contains('@') && email.len() > 3 {
            Ok(Email(email))
        } else {
            Err("Invalid email format".to_string())
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Username(String);

impl Username {
    pub fn new(username: String) -> Result<Self, String> {
        if username.len() >= 3 && username.len() <= 30 {
            Ok(Username(username))
        } else {
            Err("Username must be 3-30 characters".to_string())
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
