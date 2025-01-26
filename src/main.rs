use std::net::SocketAddr;

use tokenomics_simulator_api::app;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    // Load environment variables
    dotenvy::dotenv().expect("Environment file not found");

    // Init tracing
    let tracing = tracing_subscriber::fmt()
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env());

    if std::env::var("ENVIRONMENT")
        .and_then(|env| Ok(env == "local"))
        .is_ok()
    {
        tracing.init();
    } else {
        tracing.json().init();
    }

    let address_app = SocketAddr::from(([0, 0, 0, 0], 3000));
    info!("Server is running on {}", address_app);

    let listener = TcpListener::bind(&address_app).await.unwrap();
    axum::serve(listener, app().await.into_make_service())
        .await
        .unwrap();
}
