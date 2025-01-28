use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use tokenomics_simulator::{Simulation, SimulationBuilder, Token};
use tracing::error;
use uuid::Uuid;

use crate::{AppState, Exception};

/// Get a simulation.
///
/// # Arguments
///
/// * `state` - State of the application.
/// * `id` - ID of the simulation.
///
/// # Returns
///
/// Simulation.
pub async fn get(
    State(state): State<AppState>,
    Path((token_id, id)): Path<(Uuid, Uuid)>,
) -> impl IntoResponse {
    // Check if the token exists
    if !state
        .redis
        .write()
        .await
        .exists(&token_id.to_string())
        .await
    {
        error!("Token not found: {:?}", token_id);
        return Exception::TokenNotFound.into_response();
    }

    match state
        .redis
        .write()
        .await
        .get::<String>(&id.to_string())
        .await
    {
        Some(data) => match serde_json::from_str::<Simulation>(&data) {
            Ok(token) => (StatusCode::OK, Json(token)).into_response(),
            Err(err) => {
                error!("Failed to deserialize simulation: {:?}", err);
                Exception::InternalError.into_response()
            }
        },
        None => {
            error!("Simulation not found: {:?}", id);
            Exception::TokenNotFound.into_response()
        }
    }
}

/// Create a new simulation.
///
/// # Arguments
///
/// * `state` - State of the application.
/// * `data` - Simulation data.
///
/// # Returns
///
/// Created simulation.
pub async fn create(
    State(state): State<AppState>,
    Path(token_id): Path<String>,
    Json(data): Json<SimulationBuilder>,
) -> impl IntoResponse {
    let token = match state.redis.write().await.get::<String>(&token_id).await {
        Some(data) => match serde_json::from_str::<Token>(&data) {
            Ok(token) => token,
            Err(err) => {
                error!("Failed to deserialize token: {:?}", err);
                return Exception::InternalError.into_response();
            }
        },
        None => {
            error!("Token not found: {:?}", token_id);
            return Exception::TokenNotFound.into_response();
        }
    };

    let mut builder = Simulation::builder().token(token);

    if let Some(name) = data.name {
        builder = builder.name(name);
    }

    if let Some(description) = data.description {
        builder = builder.description(description);
    }

    if let Some(options) = data.options {
        builder = builder.options(options);
    }

    let mut simulation = match builder.build() {
        Ok(simulation) => simulation,
        Err(err) => {
            error!("Failed to build simulation: {:?}", err);
            return Exception::InvalidInput.into_response();
        }
    };

    match simulation.run() {
        Ok(_) => match state
            .redis
            .write()
            .await
            .set(
                &simulation.id.to_string(),
                serde_json::to_string(&simulation).unwrap(),
                None,
            )
            .await
        {
            Ok(_) => (StatusCode::CREATED, Json(simulation)).into_response(),
            Err(err) => {
                error!("Failed to create simulation: {:?}", err);
                err.into_response()
            }
        },
        Err(err) => {
            error!("Failed to run simulation: {:?}", err);
            Exception::InternalError.into_response()
        }
    }
}

/// Delete a simulation.
///
/// # Arguments
///
/// * `state` - State of the application.
/// * `id` - ID of the simulation.
///
/// # Returns
///
/// Result of the operation.
pub async fn delete(
    State(state): State<AppState>,
    Path((token_id, id)): Path<(Uuid, Uuid)>,
) -> impl IntoResponse {
    // Check if the token exists
    if !state
        .redis
        .write()
        .await
        .exists(&token_id.to_string())
        .await
    {
        error!("Token not found: {:?}", token_id);
        return Exception::TokenNotFound.into_response();
    }

    match state.redis.write().await.delete(&id.to_string()).await {
        Ok(result) => match result > 0 {
            true => (StatusCode::OK, "OK").into_response(),
            false => {
                error!("Simulation not found: {:?}", id);
                 Exception::SimulationNotFound.into_response()
            }
        },
        Err(err) => {
            error!(
                "Failed to delete simulation: {:?} with error: {:?}",
                id, err
            );
            Exception::InternalError.into_response()
        }
    }
}
