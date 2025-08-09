mod api;
mod models;

use axum::Router;
use std::net::SocketAddr;
use tracing::{Level, info};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    // Set up tracing for structured logging. This is a best practice.
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_env_filter("rsvp_please=info,tower_http=debug")
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set up tracing subscriber");

    info!("Starting 'rsvp-please' API server...");

    let api_router = api::create_router();

    // The main router for the application. We nest the API router under the /api path.
    let app = Router::new().nest("/api", api_router);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    info!("Server is listening on http://{}", addr);

    // This is the updated syntax. It takes the listener and the app directly.
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
