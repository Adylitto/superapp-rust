use axum::{
    routing::{get, post},
    Router,
    extract::State,
};
use tower_http::{
    cors::CorsLayer,
    trace::TraceLayer,
    compression::CompressionLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod routes;
mod middleware;
mod handlers;
mod error;

use app_storage::{PostgresRepository, repositories::user_repository::UserRepositoryImpl};
use app_auth::AuthService;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "app_api=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load environment variables
    dotenvy::dotenv().ok();

    // Initialize database connection
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://superapp:password@localhost:5432/superapp".to_string());
    
    let pool = sqlx::PgPool::connect(&database_url).await
        .expect("Failed to connect to database");
    
    // Run migrations
    sqlx::migrate!("./crates/app-storage/migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");
    
    let postgres_repo = PostgresRepository::new(pool);
    let user_repo = UserRepositoryImpl::new(postgres_repo.pool().clone());
    
    // Initialize auth service
    let jwt_secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "super_secret_jwt_key".to_string());
    let auth_service = AuthService::new(jwt_secret, user_repo);

    // Build application with state
    let app = create_app(auth_service);

    // Run server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await?;

    tracing::info!(url = "http://{}", listener.local_addr()?, "🚀 Server listening");

    axum::serve(listener, app).await?;

    Ok(())
}

fn create_app(auth_service: AuthService) -> Router {
    Router::new()
        .route("/health", get(handlers::health::health_check))
        .route("/api/v1/auth/register", post(handlers::auth::register))
        .route("/api/v1/auth/login", post(handlers::auth::login))
        .route("/api/v1/social/posts", post(handlers::social::create_post))
        .route("/api/v1/social/feed", get(handlers::social::get_feed))
        .route("/api/v1/rides/request", post(handlers::mobility::request_ride))
        .route("/api/v1/dao/proposals", post(handlers::dao::create_proposal))
        .layer(CompressionLayer::new())
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(auth_service)
}