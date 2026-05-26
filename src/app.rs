use crate::app_config::AppConfig;
use crate::app_state::{AppState, ServerState};
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
use std::sync::{Arc, Mutex};
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
            server_state: Arc::new(Mutex::new(ServerState::Offline))
    };
    spawn_status_monitor(app_state.clone());
    
    let governor_conf = GovernorConfigBuilder::default()
        .per_second(12)
        .burst_size(5)
        .finish()
        .unwrap();

    // Routes with rate limiting
    let limited_router = Router::new()
        .route("/start", post(start))
        .route("/stop", post(stop))
        .layer(tower_governor::GovernorLayer::new(governor_conf))
        .layer(
            TraceLayer::new_for_http()
            .make_span_with(DefaultMakeSpan::new().include_headers(false))
            .on_response(DefaultOnResponse::new().level(Level::INFO))
            .on_failure(DefaultOnFailure::new().level(Level::ERROR)),
        );

    Router::new()
        .route("/", get(dashboard))
        .route("/status", get(status))
        .merge(limited_router)
        .nest_service(
            "/assets",
            ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap())),
        )
        .with_state(app_state)
}

async fn dashboard(State(state): State<AppState>) -> impl IntoResponse {
    HtmlTemplate(DashboardTemplate {
        state: *state.server_state.lock().unwrap(),
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

    if *&state.server_state.lock().unwrap().eq(&ServerState::Running) {
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
            
            let mut guard: std::sync::MutexGuard<'_, ServerState> = state.server_state.lock().unwrap();
            *guard = ServerState::Starting;

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
        state: *state.server_state.lock().unwrap()
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

    if *&state.server_state.lock().unwrap().eq(&ServerState::Offline) {
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

            let mut guard  = state.server_state.lock().unwrap();
            *guard = ServerState::Stopping;

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