use axum::{
    routing::{delete, get, post},
    Router,
};

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
