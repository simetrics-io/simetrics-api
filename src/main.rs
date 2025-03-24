use std::net::SocketAddr;

use simetrics_api::app;
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_target(false).init();

    let address_app = SocketAddr::from(([0, 0, 0, 0], 8080));
    info!("Server is running on {}", address_app);

    let listener = TcpListener::bind(&address_app).await.unwrap();
    axum::serve(listener, app().await.into_make_service())
        .await
        .unwrap();
}
