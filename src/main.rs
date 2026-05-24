pub mod app;
pub mod app_config;
pub mod app_state;
pub mod template;
pub mod script_util;
pub mod status_monitor;

use std::net::SocketAddr;

use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{app::app, app_config::AppConfig};

#[tokio::main]
async fn main() {
    // logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "mcko=debug,tower_http=warn,axum=warn".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app_config = AppConfig::new();
    let app = app(&app_config);
    // configure server
    let port = app_config.server_port;
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap();

    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
}