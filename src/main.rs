use std::net::SocketAddr;

use axum::{
    routing::{delete, get, post},
    Router,
};
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::EnvFilter;

mod health;
mod token;

pub async fn app() -> Router {
    Router::new()
        .route("/health", get(health::get))
        // Tokens
        .route("/tokens", get(token::get_all))
        .route("/tokens/{id}", get(token::get))
        .route("/tokens", post(token::create))
        .route("/tokens/{id}", delete(token::delete))
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Environment file not found");
    let tracing = tracing_subscriber::fmt()
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env());

    if std::env::var("ENVIRONMENT")
        .and_then(|env| Ok(env == "local"))
        .is_ok()
    {
        tracing.init();
    } else {
        tracing.json().init();
    }

    let address_app = SocketAddr::from(([0, 0, 0, 0], 3000));

    info!("Server is running on {}", address_app);

    let listener = TcpListener::bind(&address_app).await.unwrap();
    axum::serve(listener, app().await.into_make_service())
        .await
        .unwrap();
}
