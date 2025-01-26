use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use tokenomics_simulator::Token;
use tracing::error;
use uuid::Uuid;

use crate::{AppState, Exception};

pub async fn get_all() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}

pub async fn get(State(state): State<AppState>, Path(id): Path<Uuid>) -> impl IntoResponse {
    let a = sqlx::query_as!(Token, "SELECT * FROM tokens WHERE id = $1", id);
    match a.fetch_one(&state.db).await {
        Ok(token) => (StatusCode::OK, Json(token)).into_response(),
        Err(err) => {
            error!("Error fetching token: {:?}", err);
            return Exception::TokenNotFound.into_response();
        }
    }

    // (StatusCode::OK, "OK").into_response()
}

pub async fn create() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}

pub async fn delete(Path(id): Path<String>) -> impl IntoResponse {
    println!("id: {}", id);

    (StatusCode::OK, "OK")
}
