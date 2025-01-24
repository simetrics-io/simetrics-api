use axum::{http::StatusCode, response::IntoResponse};

/// Get the health status.
///
/// # Returns
///
/// The successful health status.
pub async fn get() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_check_ok() {
        let response = get().await.into_response();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
