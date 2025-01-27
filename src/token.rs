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

// TODO: How to get all? we need authorization or something?
pub async fn get_all() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}

pub async fn get(State(state): State<AppState>, Path(id): Path<Uuid>) -> impl IntoResponse {
    match state
        .redis
        .write()
        .await
        .get::<String>(&id.to_string())
        .await
    {
        Some(data) => match serde_json::from_str::<Token>(&data) {
            Ok(token) => (StatusCode::OK, Json(token)).into_response(),
            Err(err) => {
                error!("Failed to deserialize token: {:?}", err);
                Exception::InternalError.into_response()
            }
        },
        None => {
            error!("Token not found: {:?}", id);
            Exception::TokenNotFound.into_response()
        }
    }
}

pub async fn create() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}

pub async fn delete(State(state): State<AppState>, Path(id): Path<String>) -> impl IntoResponse {
    match state.redis.write().await.delete(&id).await {
        Ok(result) => match result > 0 {
            true => (StatusCode::OK, "OK").into_response(),
            false => {
                error!("Failed to delete token: {:?}", id);
                return Exception::TokenNotFound.into_response();
            }
        },
        Err(err) => {
            error!("Failed to delete token: {:?} with error: {:?}", id, err);
            Exception::InternalError.into_response()
        }
    }
}
