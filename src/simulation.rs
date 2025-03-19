use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use rust_decimal::Decimal;
use tokenomics_simulator::{
    Simulation, SimulationBuilder, SimulationInterval, SimulationOptions, Token,
};
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
        Some(token) => match validate_token(&token) {
            Ok(_) => token,
            Err(err) => return err.into_response(),
        },
        _ => return Exception::TokenNotFound.into_response(),
    };

    let mut simulation_builder = Simulation::builder().token(token);

    if let Some(name) = data.name {
        simulation_builder = simulation_builder.name(name);
    }

    if let Some(description) = data.description {
        simulation_builder = simulation_builder.description(description);
    }

    if let Some(options) = data.options {
        if let Err(err) = validate_simulation(&options) {
            return err.into_response();
        }

        simulation_builder = simulation_builder.options(options);
    }

    let mut simulation = match simulation_builder.build() {
        Ok(simulation) => simulation,
        Err(err) => {
            error!("Failed to build simulation: {:?}", err);
            return Exception::InternalError.into_response();
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

fn validate_token(token: &Token) -> Result<(), Exception> {
    if let Some(airdrop) = token.airdrop_percentage {
        if airdrop <= Decimal::default() || airdrop > Decimal::new(100, 0) {
            return Err(Exception::ValidationFailed(
                "Airdrop percentage must be more 0 and less than or equal to 100.".to_string(),
            ));
        }
    }

    if token.initial_supply_percentage <= Decimal::default()
        || token.initial_supply_percentage > Decimal::new(100, 0)
    {
        return Err(Exception::ValidationFailed(
            "Initial supply percentage must be more than 0 and less than or equal to 100."
                .to_string(),
        ));
    }

    Ok(())
}

/// Validate the simulation input data.
///
/// # Arguments
///
/// * `simulation` - Simulation input data.
///
/// # Returns
///
/// Result of the validation.
fn validate_simulation(simulation: &SimulationOptions) -> Result<(), Exception> {
    if simulation.total_users < 1 || simulation.total_users > 100000 {
        return Err(Exception::ValidationFailed(
            "Total users must be more than 0 and less than or equal to 100000.".to_string(),
        ));
    }

    if simulation.decimal_precision < 1 || simulation.decimal_precision > 18 {
        return Err(Exception::ValidationFailed(
            "Decimal precision must be more than or equal to 0 and less than or equal to 18."
                .to_string(),
        ));
    }

    match simulation.interval_type {
        SimulationInterval::Daily => {
            if simulation.duration < 1 || simulation.duration > 365 {
                return Err(Exception::ValidationFailed(
                    "Duration value must be more than 0 and less than or equal to 365.".to_string(),
                ));
            }
        }
        SimulationInterval::Hourly => {
            if simulation.duration < 1 || simulation.duration > 24 {
                return Err(Exception::ValidationFailed(
                    "Duration value must be more than 0 and less than or equal to 24.".to_string(),
                ));
            }
        }
        SimulationInterval::Weekly => {
            if simulation.duration < 1 || simulation.duration > 52 {
                return Err(Exception::ValidationFailed(
                    "Duration value must be more than 0 and less than or equal to 52.".to_string(),
                ));
            }
        }
        SimulationInterval::Monthly => {
            if simulation.duration < 1 || simulation.duration > 12 {
                return Err(Exception::ValidationFailed(
                    "Duration value must be more than 0 and less than or equal to 12.".to_string(),
                ));
            }
        }
    };

    if simulation.market_volatility < Decimal::default()
        || simulation.market_volatility > Decimal::new(1, 0)
    {
        return Err(Exception::ValidationFailed(
            "Market volatility must be more than or equal to 0 and less than or equal to 1."
                .to_string(),
        ));
    }

    if let Some(fee) = simulation.transaction_fee_percentage {
        if fee <= Decimal::default() || fee > Decimal::new(1, 0) {
            return Err(Exception::ValidationFailed(
                "Transaction fee percentage must be more than 0 and less than or equal to 100."
                    .to_string(),
            ));
        }
    }

    Ok(())
}
