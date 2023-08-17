use axum::{
    extract::Extension, routing::{get, post}, Router,
};

use sqlx::sqlite::SqlitePoolOptions;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod errors;
mod models;
mod controllers;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = "sqlite://./database.db".to_string();
    
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("tower_http=trace")
                .unwrap_or_else(|_| "example_tracing_aka_logging=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let pool = SqlitePoolOptions::new()
        .max_connections(50)
        .connect(&database_url)
        .await?;

    let app = Router::new()
        .route("/:id", get(controllers::url::get_url))
        .route("/new-url", post(controllers::url::create_short_url))
        .layer(Extension(pool))
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([127, 0, 0, 1], 80)); 
    tracing::debug!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
