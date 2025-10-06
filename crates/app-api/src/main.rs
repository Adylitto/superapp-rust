use axum::{
    routing::{get, post},
    Router,
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

    // Build application
    let app = create_app();

    // Run server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await?;

    tracing::info!("🚀 Server listening on {}", listener.local_addr()?);

    axum::serve(listener, app).await?;

    Ok(())
}

fn create_app() -> Router {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_health_check() {
        let app = create_app();
        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .uri("/health")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
