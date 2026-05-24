use crate::app_config::AppConfig;
use crate::app_state::AppState;
use crate::template::{DashboardTemplate, HtmlTemplate, StatusBoxTemplate};
use axum::extract::State;
use axum::http::{StatusCode, header};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Form, Router};
use serde::Deserialize;
use std::env;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpStream;
use tower_http::services::ServeDir;
use tracing::info;
use crate::script_util::run_script;

#[derive(Deserialize)]
struct FormData {
    secret: String,
}

// Web Service
pub fn app(app_config: &AppConfig) -> Router {
    // assets (css)
    let assets_path = env::current_dir().unwrap();

    Router::new()
        .route("/", get(dashboard))
        .route("/start", post(start))
        .route("/stop", post(stop))
        .route("/status", get(status))
        .nest_service(
            "/assets",
            ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap())),
        )
        .with_state(AppState {
            config: Arc::new(app_config.clone()),
        })
}

async fn dashboard(State(state): State<AppState>) -> impl IntoResponse {
    HtmlTemplate(DashboardTemplate {
        running: is_server_running(&state.config.mc_port).await,
    })
}

async fn start(State(state): State<AppState>, Form(data): Form<FormData>) -> impl IntoResponse {
    let env_secret = match env::var("MC_SECRET") {
        Ok(value) => value,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                [(header::CACHE_CONTROL, "no-store")],
                r#"<div class="error">Server secret is not configured.</div>"#,
            );
        }
    };

    if data.secret != env_secret {
        return (
            StatusCode::FORBIDDEN,
            [(header::CACHE_CONTROL, "no-store")],
            r#"<div class="error">Invalid secret.</div>"#,
        );
    }

    if is_server_running(&state.config.mc_port).await {
        info!("MC Server already running...");
        return (
            StatusCode::OK,
            [(header::CACHE_CONTROL, "no-store")],
            r#"<div class="success">Server is already running.</div>"#,
        );
    }

    let root = PathBuf::from(".");
    let script_path = root.join("scripts").join(&state.config.mc_start_script);

    match run_script(script_path.as_path()).await {
        Ok(_) => {
            info!("MC server start command spawned");

            (
                StatusCode::OK,
                [(header::CACHE_CONTROL, "no-store")],
                r#"<div class="success">Server start requested.</div>"#,
            )
        }
        Err(err) => {
            tracing::error!("Failed to execute command: {err}");

            (
                StatusCode::INTERNAL_SERVER_ERROR,
                [(header::CACHE_CONTROL, "no-store")],
                r#"<div class="error">Failed to start server.</div>"#,
            )
        }
    }
}

async fn status(State(state): State<AppState>) -> impl IntoResponse {
    HtmlTemplate(StatusBoxTemplate {
        running: is_server_running(&state.config.mc_port).await,
    })
}

async fn stop(State(state): State<AppState>, Form(data): Form<FormData>) -> impl IntoResponse {
    let env_secret = match env::var("MC_SECRET") {
        Ok(value) => value,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                [(header::CACHE_CONTROL, "no-store")],
                r#"<div class="error">Server secret is not configured.</div>"#,
            );
        }
    };

    if data.secret != env_secret {
        return (
            StatusCode::FORBIDDEN,
            [(header::CACHE_CONTROL, "no-store")],
            r#"<div class="error">Invalid secret.</div>"#,
        );
    }

    if !is_server_running(&state.config.mc_port).await {
        info!("MC Server is not running...");
        return (
            StatusCode::OK,
            [(header::CACHE_CONTROL, "no-store")],
            r#"<div class="success">Server is not running.</div>"#,
        );
    }

    let root = PathBuf::from(".");
    let script_path = root.join("scripts").join(&state.config.mc_stop_script);

    match run_script(script_path.as_path()).await {
        Ok(_) => {
            info!("MC server stop command spawned");

            (
                StatusCode::OK,
                [(header::CACHE_CONTROL, "no-store")],
                r#"<div class="success">Server stop requested.</div>"#,
            )
        }
        Err(err) => {
            tracing::error!("Failed to execute command: {err}");
            
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                [(header::CACHE_CONTROL, "no-store")],
                r#"<div class="error">Failed to stop server.</div>"#,
            )
        }
    }
}

async fn is_server_running(port: &str) -> bool {
    let addr: SocketAddr = format!("127.0.0.1:{port}")
        .parse()
        .expect("invalid address");

    tokio::time::timeout(Duration::from_secs(1), TcpStream::connect(addr))
        .await
        .is_ok()
}
