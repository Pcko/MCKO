pub mod app;
pub mod app_config;
pub mod model;
pub mod process_util;
pub mod rcon_client;
pub mod script_util;
pub mod setup;
pub mod status_monitor;

use std::net::SocketAddr;

use anyhow::Context;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{app::app, app_config::AppConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "mcko=debug,tower_http=warn,axum=warn".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app_config = AppConfig::new();

    match std::env::args().nth(1).as_deref() {
        Some("setup") => {
            setup::run_setup(&app_config).await?;
            Ok(())
        }
        None => run_web_server(app_config).await,
        Some(command) => {
            anyhow::bail!("unknown command: {command}");
        }
    }
}

async fn run_web_server(config: AppConfig) -> anyhow::Result<()> {
    let app: axum::Router = app(&config);

    // configure server
    let host = config.server_host;
    let port = config.server_port;
    let bind_addr = format!("{host}:{port}");

    let listener = tokio::net::TcpListener::bind(format!("{host}:{port}"))
        .await
        .with_context(|| format!("failed to bind web server to {bind_addr}"))?;

    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .context("web server failed")?;

    Ok(())
}
