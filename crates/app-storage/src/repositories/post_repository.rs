use app_core::{Post, PostVisibility};
use sqlx::{PgPool, FromRow};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(FromRow, Debug)]
struct PostDb {
    id: Uuid,
    author_id: Uuid,
    content: String,
    media_urls: serde_json::Value,
    likes_count: i32,
    comments_count: i32,
    shares_count: i32,
    visibility: String,
    is_ai_moderated: bool,
    moderation_score: f64,
    tokens_earned: i64,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<PostDb> for Post {
    fn from(db_post: PostDb) -> Self {
        Post {
            id: db_post.id,
            author_id: db_post.author_id,
            content: db_post.content,
            media_urls: serde_json::from_value(db_post.media_urls).unwrap_or_else(|_| vec![]),
            likes_count: db_post.likes_count as u32,
            comments_count: db_post.comments_count as u32,
            shares_count: db_post.shares_count as u32,
            visibility: match db_post.visibility.as_str() {
                "Private" => PostVisibility::Private,
                "Friends" => PostVisibility::Friends,
                _ => PostVisibility::Public,
            },
            is_ai_moderated: db_post.is_ai_moderated,
            moderation_score: db_post.moderation_score,
            tokens_earned: db_post.tokens_earned as u64,
            created_at: db_post.created_at,
            updated_at: db_post.updated_at,
        }
    }
}

pub struct PostRepositoryImpl {
    pool: PgPool,
}

impl PostRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_id(&self, id: Uuid) -> app_core::Result<Option<Post>> {
        let db_post = sqlx::query_as!(PostDb,
            "SELECT * FROM posts WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| app_core::Error::Internal(e.into()))?;

        Ok(db_post.map(|db_post| db_post.into()))
    }

    pub async fn find_by_author(&self, author_id: Uuid) -> app_core::Result<Vec<Post>> {
        let db_posts = sqlx::query_as!(PostDb,
            "SELECT * FROM posts WHERE author_id = $1 ORDER BY created_at DESC",
            author_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| app_core::Error::Internal(e.into()))?;

        Ok(db_posts.into_iter().map(|db_post| db_post.into()).collect())
    }

    pub async fn create(&self, post: &Post) -> app_core::Result<()> {
        sqlx::query!(
            "INSERT INTO posts (id, author_id, content, media_urls, likes_count, comments_count, shares_count, visibility, is_ai_moderated, moderation_score, tokens_earned, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)",
            post.id,
            post.author_id,
            &post.content,
            serde_json::to_value(&post.media_urls)?,
            post.likes_count as i32,
            post.comments_count as i32,
            post.shares_count as i32,
            match post.visibility {
                PostVisibility::Public => "Public",
                PostVisibility::Friends => "Friends",
                PostVisibility::Private => "Private",
            },
            post.is_ai_moderated,
            post.moderation_score,
            post.tokens_earned as i64,
            post.created_at,
            post.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| app_core::Error::Internal(e.into()))?;

        Ok(())
    }

    pub async fn update(&self, post: &Post) -> app_core::Result<()> {
        sqlx::query!(
            "UPDATE posts SET 
                content = $1,
                media_urls = $2,
                likes_count = $3,
                comments_count = $4,
                shares_count = $5,
                visibility = $6,
                is_ai_moderated = $7,
                moderation_score = $8,
                tokens_earned = $9,
                updated_at = $10
             WHERE id = $11",
            &post.content,
            serde_json::to_value(&post.media_urls)?,
            post.likes_count as i32,
            post.comments_count as i32,
            post.shares_count as i32,
            match post.visibility {
                PostVisibility::Public => "Public",
                PostVisibility::Friends => "Friends",
                PostVisibility::Private => "Private",
            },
            post.is_ai_moderated,
            post.moderation_score,
            post.tokens_earned as i64,
            post.updated_at,
            post.id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| app_core::Error::Internal(e.into()))?;

        Ok(())
    }

    pub async fn delete(&self, id: Uuid) -> app_core::Result<()> {
        sqlx::query!("DELETE FROM posts WHERE id = $1", id)
            .execute(&self.pool)
            .await
            .map_err(|e| app_core::Error::Internal(e.into()))?;

        Ok(())
    }

    pub async fn increment_likes(&self, post_id: Uuid) -> app_core::Result<()> {
        sqlx::query!("UPDATE posts SET likes_count = likes_count + 1, updated_at = NOW() WHERE id = $1", post_id)
            .execute(&self.pool)
            .await
            .map_err(|e| app_core::Error::Internal(e.into()))?;

        Ok(())
    }
}