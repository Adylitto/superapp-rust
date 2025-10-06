use app_storage::repositories::{user_repository::UserRepositoryImpl, post_repository::PostRepositoryImpl};
use app_core::{User, Post, PostVisibility};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing database connection...");
    
    // Initialize database connection
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres:postgres@localhost:54322/postgres".to_string());
    
    let pool = sqlx::PgPool::connect(&database_url).await?;
    
    println!("✅ Connected to database successfully!");
    
    // Test user repository
    let user_repo = UserRepositoryImpl::new(pool.clone());
    
    // Create a test user
    let user_id = Uuid::new_v4();
    let user = User::new(
        format!("test{}@example.com", user_id),
        format!("testuser_{}", user_id),
        "hashed_password_example".to_string(),
    );
    
    println!("📝 Creating test user...");
    user_repo.create(&user).await?;
    println!("✅ User created successfully!");
    
    // Retrieve the user
    let retrieved_user = user_repo.find_by_id(user.id).await?;
    println!("🔍 Retrieved user: {:?}", retrieved_user.is_some());
    
    // Test post repository
    let post_repo = PostRepositoryImpl::new(pool);
    
    // Create a test post
    let post = Post::new(
        user.id,
        "Hello from SuperApp!".to_string(),
        PostVisibility::Public,
    );
    
    println!("📝 Creating test post...");
    post_repo.create(&post).await?;
    println!("✅ Post created successfully!");
    
    // Retrieve the post
    let retrieved_post = post_repo.find_by_id(post.id).await?;
    println!("🔍 Retrieved post: {:?}", retrieved_post.is_some());
    
    println!("\n🎉 All tests passed! Database integration is working correctly.");
    Ok(())
}