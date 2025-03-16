use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use tokenomics_simulator::{Simulation, SimulationBuilder};
use tracing::error;

use crate::Exception;

/// Create a new simulation.
///
/// # Arguments
///
/// * `data` - Simulation input data.
///
/// # Returns
///
/// Created simulation.
pub async fn create(Json(data): Json<SimulationBuilder>) -> impl IntoResponse {
    let token = match data.token {
        Some(token) => token,
        _ => return Exception::TokenNotFound.into_response(),
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
        Ok(_) => (StatusCode::CREATED, Json(simulation)).into_response(),
        Err(err) => {
            error!("Failed to run simulation: {:?}", err);
            Exception::InternalError.into_response()
        }
    }
}
