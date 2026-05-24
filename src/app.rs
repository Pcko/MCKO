use crate::app_config::AppConfig;
use crate::app_state::AppState;
use crate::script_util::run_script;
use crate::status_monitor::spawn_status_monitor;
use crate::template::{DashboardTemplate, HtmlTemplate, StatusBoxTemplate};
use axum::extract::State;
use axum::http::{StatusCode, header};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Form, Router};
use serde::Deserialize;
use tower_governor::governor::GovernorConfigBuilder;
use tower_http::services::ServeDir;
use tower_http::trace::{DefaultMakeSpan, DefaultOnFailure, DefaultOnResponse, TraceLayer};
use std::env;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tracing::{Level, error, info};
use argon2::{PasswordVerifier};

#[derive(Deserialize)]
struct FormData {
    secret: String,
}

// Web Service
pub fn app(app_config: &AppConfig) -> Router {
    // assets (css)
    let assets_path = env::current_dir().unwrap();

    // status monitor
    let app_state = AppState {
            config: Arc::new(app_config.clone()),
            server_running: Arc::new(AtomicBool::new(false))
    };
    spawn_status_monitor(app_state.clone());
    
    let governor_conf = GovernorConfigBuilder::default()
        .per_second(300)
        .burst_size(5)
        .finish()
        .unwrap();

    // Routes with rate limiting
    let limited_router = Router::new()
        .route("/start", post(start))
        .route("/stop", post(stop))
        .layer(tower_governor::GovernorLayer::new(governor_conf));

    Router::new()
        .route("/", get(dashboard))
        .route("/status", get(status))
        .merge(limited_router)
        .nest_service(
            "/assets",
            ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap())),
        )
        .layer(
            TraceLayer::new_for_http()
            .make_span_with(DefaultMakeSpan::new().include_headers(false))
            .on_response(DefaultOnResponse::new().level(Level::INFO))
            .on_failure(DefaultOnFailure::new().level(Level::ERROR)),
        )
        .with_state(app_state)
}

async fn dashboard(State(state): State<AppState>) -> impl IntoResponse {
    HtmlTemplate(DashboardTemplate {
        running: state.server_running.load(Ordering::Relaxed),
    })
}

async fn start(State(state): State<AppState>, Form(data): Form<FormData>) -> impl IntoResponse {
    if !verify_secret(&data.secret, &state.config.secret_hash) {
        return (
            StatusCode::FORBIDDEN,
            [(header::CACHE_CONTROL, "no-store")],
            r#"<div class="error">Invalid secret.</div>"#,
        );
    }

    if state.server_running.load(Ordering::Relaxed) {
        info!("MC Server already running...");
        return (
            StatusCode::OK,
            [(header::CACHE_CONTROL, "no-store")],
            r#"<div class="success">Server is already running.</div>"#,
        );
    }

    let script_path: PathBuf = PathBuf::from(&state.config.mc_start_script);

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
            error!("Failed to execute command: {err}");

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
        running: state.server_running.load(Ordering::Relaxed),
    })
}

async fn stop(State(state): State<AppState>, Form(data): Form<FormData>) -> impl IntoResponse {

    if !verify_secret( &data.secret,&state.config.secret_hash) {
        return (
            StatusCode::FORBIDDEN,
            [(header::CACHE_CONTROL, "no-store")],
            r#"<div class="error">Invalid secret.</div>"#,
        );
    }

    if !state.server_running.load(Ordering::Relaxed) {
        info!("MC Server is not running...");
        return (
            StatusCode::OK,
            [(header::CACHE_CONTROL, "no-store")],
            r#"<div class="success">Server is not running.</div>"#,
        );
    }
    
    let script_path: PathBuf = PathBuf::from(&state.config.mc_stop_script);

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
            error!("Failed to execute command: {err}");
            
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                [(header::CACHE_CONTROL, "no-store")],
                r#"<div class="error">Failed to stop server.</div>"#,
            )
        }
    }
}

fn verify_secret(provided_secret: &str, expected_hash: &str) -> bool {
    let parsed_hash = match argon2::PasswordHash::new(expected_hash) {
        Ok(hash) => hash,
        Err(err) => {
            error!("Failed to parse secret hash: {err}");
            return false;
        },
    };
    
    argon2::Argon2::default()
        .verify_password(provided_secret.as_bytes(), &parsed_hash)
        .is_ok()
}