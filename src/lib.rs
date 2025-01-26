use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{delete, get, post},
    Json, Router,
};
use serde::Serialize;
use serde_json::json;
use serde_variant::to_variant_name;
use sqlx::{postgres::PgPoolOptions, PgPool};
use strum::EnumProperty;
use strum_macros::{EnumIter, EnumProperty};
use thiserror::Error;
use tracing::debug;

mod health;
mod token;

/// Application state.
#[derive(Clone, Debug)]
pub struct AppState {
    /// Postgres connection pool.
    pub db: PgPool,
}

/// Exceptions that can be thrown by the application.
#[derive(Debug, Error, Serialize, PartialEq, Eq, EnumProperty, EnumIter)]
pub enum Exception {
    /// Token not found.
    #[serde(rename = "TOKEN_NOT_FOUND")]
    #[error("Token not found.")]
    #[strum(props(status_code = "404"))]
    TokenNotFound,
}

impl Exception {
    /// Gets the status code of the exception as u16.
    ///
    /// # Returns
    ///
    /// The status code of the exception as u16.
    pub fn status_code_u16(&self) -> u16 {
        let status_code = self.get_str("status_code").unwrap();
        status_code.parse::<u16>().unwrap()
    }

    /// Gets the status code of the exception as StatusCode enum.
    ///
    /// # Returns
    ///
    /// The status code of the exception as StatusCode enum.
    pub fn status_code(&self) -> StatusCode {
        let status_code_u16 = self.status_code_u16();
        StatusCode::from_u16(status_code_u16).unwrap()
    }

    /// Gets the variant name of the exception.
    ///
    /// # Returns
    ///
    /// The variant name of the exception.
    pub fn get_variant_name(&self) -> &str {
        to_variant_name(self).unwrap()
    }
}

impl IntoResponse for Exception {
    /// Converts the `Exception` into an HTTP response.
    ///
    /// # Returns
    ///
    /// A `Response` containing the appropriate status code and JSON representation of the exception.
    fn into_response(self) -> Response {
        let message = &self.to_string();
        let status_code = self.status_code();
        let code = self.get_variant_name();

        (
            status_code,
            Json(json!({ "code": code, "message": message })),
        )
            .into_response()
    }
}

pub async fn app() -> Router {
    // Connect to Postgres
    debug!("Connecting to Postgres ...");
    let db = PgPoolOptions::new()
        .max_connections(20)
        .connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL is missed"))
        .await
        .expect("cannot connect to postgresql");
    debug!("Connected to Postgres");

    // Run migrations
    debug!("Running migrations ...");
    sqlx::migrate!()
        .run(&db)
        .await
        .expect("cannot run migrations");
    debug!("Migrations are run");

    let state = AppState { db };

    Router::new()
        .route("/health", get(health::get))
        // Tokens
        .route("/tokens", get(token::get_all))
        .route("/tokens/{id}", get(token::get))
        .route("/tokens", post(token::create))
        .route("/tokens/{id}", delete(token::delete))
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use super::Exception;

    #[test]
    fn test_status_code_u16() {
        for el in Exception::iter() {
            el.status_code_u16();
        }
    }

    #[test]
    fn test_status_code() {
        for el in Exception::iter() {
            el.status_code();
        }
    }

    #[test]
    fn test_variant_name() {
        for el in Exception::iter() {
            el.get_variant_name();
        }
    }
}
