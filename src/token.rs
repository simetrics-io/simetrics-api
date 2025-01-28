use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use tokenomics_simulator::{Token, TokenBuilder};
use tracing::error;
use uuid::Uuid;

use crate::{AppState, Exception, CACHE_TTL};

/// Get a token.
///
/// # Arguments
///
/// * `state` - State of the application.
/// * `id` - ID of the token.
///
/// # Returns
///
/// Token.
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

/// Create a new token.
///
/// # Arguments
///
/// * `state` - State of the application.
/// * `data` - Token data.
///
/// # Returns
///
/// Created token.
pub async fn create(
    State(state): State<AppState>,
    Json(data): Json<TokenBuilder>,
) -> impl IntoResponse {
    let mut token = TokenBuilder::new();

    if let Some(name) = data.name {
        token = token.name(name);
    }

    if let Some(symbol) = data.symbol {
        token = token.symbol(symbol);
    }

    if let Some(total_supply) = data.total_supply {
        token = token.total_supply(total_supply);
    }

    if let Some(current_supply) = data.current_supply {
        token = token.current_supply(current_supply);
    }

    if let Some(initial_supply_percentage) = data.initial_supply_percentage {
        token = token.initial_supply_percentage(initial_supply_percentage);
    }

    if let Some(inflation_rate) = data.inflation_rate {
        token = token.inflation_rate(inflation_rate);
    }

    if let Some(burn_rate) = data.burn_rate {
        token = token.burn_rate(burn_rate);
    }

    if let Some(initial_price) = data.initial_price {
        token = token.initial_price(initial_price);
    }

    if let Some(airdrop_percentage) = data.airdrop_percentage {
        token = token.airdrop_percentage(airdrop_percentage);
    }

    if let Some(unlock_schedule) = data.unlock_schedule {
        token = token.unlock_schedule(unlock_schedule);
    }

    match token.build() {
        Ok(token) => match state
            .redis
            .write()
            .await
            .set(
                &token.id.to_string(),
                serde_json::to_string(&token).unwrap(),
                Some(CACHE_TTL),
            )
            .await
        {
            Ok(_) => (StatusCode::CREATED, Json(token)).into_response(),
            Err(err) => {
                error!("Failed to create token: {:?}", err);
                err.into_response()
            }
        },
        Err(err) => {
            error!("Failed to build token: {:?}", err);
            Exception::InvalidInput.into_response()
        }
    }
}

/// Delete a token.
///
/// # Arguments
///
/// * `state` - State of the application.
/// * `id` - ID of the token.
///
/// # Returns
///
/// Result of the operation.
pub async fn delete(State(state): State<AppState>, Path(id): Path<Uuid>) -> impl IntoResponse {
    match state.redis.write().await.delete(&id.to_string()).await {
        Ok(result) => match result > 0 {
            true => (StatusCode::OK, "OK").into_response(),
            false => {
                error!("Token not found: {:?}", id);
                Exception::TokenNotFound.into_response()
            }
        },
        Err(err) => {
            error!("Failed to delete token: {:?} with error: {:?}", id, err);
            Exception::InternalError.into_response()
        }
    }
}
