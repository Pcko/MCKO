mod app;
mod app_config;
mod app_state;
mod template;
pub mod script_util;

use crate::app::app;
use crate::app_config::AppConfig;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    let app_config = AppConfig::new();

    // logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = app(&app_config);
    // configure server
    let port = app_config.server_port;
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap();

    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}