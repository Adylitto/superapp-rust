use app_core::{User, Result as CoreResult, user::UserProfile};
use sqlx::{PgPool, FromRow};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(FromRow, Debug)]
struct UserDb {
    id: Uuid,
    email: String,
    username: String,
    password_hash: String,
    profile_full_name: Option<String>,
    profile_avatar_url: Option<String>,
    profile_bio: Option<String>,
    profile_location: Option<String>,
    profile_phone: Option<String>,
    profile_preferences: serde_json::Value,
    token_balance: i64,
    reputation_score: f64,
    is_verified: bool,
    is_active: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<UserDb> for User {
    fn from(db_user: UserDb) -> Self {
        User {
            id: db_user.id,
            email: db_user.email,
            username: db_user.username,
            password_hash: db_user.password_hash,
            profile: UserProfile {
                full_name: db_user.profile_full_name.unwrap_or_default(),
                avatar_url: db_user.profile_avatar_url,
                bio: db_user.profile_bio,
                location: db_user.profile_location,
                phone: db_user.profile_phone,
                preferences: serde_json::from_value(db_user.profile_preferences).unwrap_or_default(),
            },
            token_balance: db_user.token_balance as u64,
            reputation_score: db_user.reputation_score,
            is_verified: db_user.is_verified,
            is_active: db_user.is_active,
            created_at: db_user.created_at,
            updated_at: db_user.updated_at,
        }
    }
}

pub struct UserRepositoryImpl {
    pool: PgPool,
}

impl UserRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_id(&self, id: Uuid) -> CoreResult<Option<User>> {
        let db_user = sqlx::query_as!(UserDb, 
            "SELECT * FROM users WHERE id = $1 AND is_active = true",
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| app_core::Error::Internal(e.into()))?;

        Ok(db_user.map(|db_user| db_user.into()))
    }

    pub async fn find_by_email(&self, email: &str) -> CoreResult<Option<User>> {
        let db_user = sqlx::query_as!(UserDb,
            "SELECT * FROM users WHERE email = $1 AND is_active = true",
            email
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| app_core::Error::Internal(e.into()))?;

        Ok(db_user.map(|db_user| db_user.into()))
    }

    pub async fn find_by_username(&self, username: &str) -> CoreResult<Option<User>> {
        let db_user = sqlx::query_as!(UserDb,
            "SELECT * FROM users WHERE username = $1 AND is_active = true",
            username
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| app_core::Error::Internal(e.into()))?;

        Ok(db_user.map(|db_user| db_user.into()))
    }

    pub async fn create(&self, user: &User) -> CoreResult<()> {
        sqlx::query!(
            "INSERT INTO users (id, email, username, password_hash, profile_full_name, profile_avatar_url, profile_bio, profile_location, profile_phone, profile_preferences, token_balance, reputation_score, is_verified, is_active, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)",
            user.id,
            user.email,
            user.username,
            user.password_hash,
            user.profile.full_name.as_ref(),
            user.profile.avatar_url.as_ref(),
            user.profile.bio.as_ref(),
            user.profile.location.as_ref(),
            user.profile.phone.as_ref(),
            serde_json::to_value(&user.profile.preferences)?,
            user.token_balance as i64,
            user.reputation_score,
            user.is_verified,
            user.is_active,
            user.created_at,
            user.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| app_core::Error::Internal(e.into()))?;

        Ok(())
    }

    pub async fn update(&self, user: &User) -> CoreResult<()> {
        sqlx::query!(
            "UPDATE users SET 
                email = $1, 
                username = $2, 
                password_hash = $3, 
                profile_full_name = $4, 
                profile_avatar_url = $5, 
                profile_bio = $6, 
                profile_location = $7, 
                profile_phone = $8, 
                profile_preferences = $9, 
                token_balance = $10, 
                reputation_score = $11, 
                is_verified = $12, 
                is_active = $13, 
                updated_at = $14
             WHERE id = $15",
            user.email,
            user.username,
            user.password_hash,
            user.profile.full_name.as_ref(),
            user.profile.avatar_url.as_ref(),
            user.profile.bio.as_ref(),
            user.profile.location.as_ref(),
            user.profile.phone.as_ref(),
            serde_json::to_value(&user.profile.preferences)?,
            user.token_balance as i64,
            user.reputation_score,
            user.is_verified,
            user.is_active,
            user.updated_at,
            user.id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| app_core::Error::Internal(e.into()))?;

        Ok(())
    }

    pub async fn delete(&self, id: Uuid) -> CoreResult<()> {
        sqlx::query!("UPDATE users SET is_active = false WHERE id = $1", id)
            .execute(&self.pool)
            .await
            .map_err(|e| app_core::Error::Internal(e.into()))?;

        Ok(())
    }
}