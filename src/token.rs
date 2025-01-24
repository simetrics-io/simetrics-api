use axum::{extract::Path, http::StatusCode, response::IntoResponse};
use uuid::Uuid;

pub async fn get_all() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}

pub async fn get(Path(id): Path<Uuid>) -> impl IntoResponse {
    println!("id: {}", id);

    (StatusCode::OK, "OK")
}

pub async fn create() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}

pub async fn delete(Path(id): Path<String>) -> impl IntoResponse {
    println!("id: {}", id);

    (StatusCode::OK, "OK")
}
