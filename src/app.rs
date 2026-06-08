use crate::app_config::AppConfig;
use crate::model::app_state::AppState;
use crate::model::server_state::ServerState;
use crate::model::template::{DashboardTemplate, HtmlTemplate, StatusBoxTemplate};
use crate::process_util::{get_memory_usage, get_tmux_pid};
use crate::rcon_client::RconClient;
use crate::script_util::run_script;
use crate::status_monitor::spawn_status_monitor;
use anyhow::Context;
use argon2::PasswordVerifier;
use axum::extract::State;
use axum::http::{StatusCode, header};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Form, Router};
use regex::Regex;
use serde::Deserialize;
use std::env;
use std::path::PathBuf;
use std::result::Result::Ok;
use std::sync::{Arc, Mutex, RwLock};
use std::time::Instant;
use tower_governor::governor::GovernorConfigBuilder;
use tower_http::services::ServeDir;
use tower_http::trace::{DefaultMakeSpan, DefaultOnFailure, DefaultOnResponse, TraceLayer};
use tracing::{Level, error, info};

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
        server_state: Arc::new(Mutex::new(ServerState::Offline)),
        started_at: Arc::new(RwLock::new(None)),
        rcon_client: Arc::new(RconClient::new(
            &app_config.rcon_host,
            &app_config.rcon_port,
            app_config.rcon_password.clone(),
        )),
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
        uptime: format_uptime(*state.started_at.read().unwrap()),
        port: state.config.mc_port.clone(),
        player_count: "-".to_string(),
        memory_usage: "-".to_string(),
    })
}

async fn start(State(state): State<AppState>, Form(data): Form<FormData>) -> impl IntoResponse {
    if !verify_secret(&data.secret, &state.config.secret_hash) {
        return (
            StatusCode::FORBIDDEN,
            [(header::CACHE_CONTROL, "no-store")],
            r#"<div class="error">Message: Invalid secret.</div>"#,
        );
    }

    if *state.server_state.lock().unwrap() == ServerState::Running {
        info!("MC Server already running...");
        return (
            StatusCode::OK,
            [(header::CACHE_CONTROL, "no-store")],
            r#"<div class="error">Message: Server is already running.</div>"#,
        );
    }

    let script_path: PathBuf = PathBuf::from(&state.config.mc_start_script);

    match run_script(script_path.as_path()).await {
        Ok(exit_status) => {
            let mut guard: std::sync::MutexGuard<'_, ServerState> =
                state.server_state.lock().unwrap();

            if exit_status.success() {
                info!("MC server start command spawned");
                *guard = ServerState::Starting;

                (
                    StatusCode::OK,
                    [(header::CACHE_CONTROL, "no-store")],
                    r#"<div class="success">Message: Server start requested.</div>"#,
                )
            } else {
                error!("Command panicked while running!");

                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    [(header::CACHE_CONTROL, "no-store")],
                    r#"<div class="error">Message: Failed to start server.</div>"#,
                )
            }
        }
        Err(err) => {
            error!("Failed to execute command: {err}");

            (
                StatusCode::INTERNAL_SERVER_ERROR,
                [(header::CACHE_CONTROL, "no-store")],
                r#"<div class="error">Message: Failed to start server.</div>"#,
            )
        }
    }
}

async fn status(State(state): State<AppState>) -> impl IntoResponse {
    let player_count = get_player_count(&state).await.unwrap_or("-".to_string());
    let uptime = format_uptime(*state.started_at.read().unwrap());
    let mut memory_usage = "-".to_string();

    // TODO replace tmux on windows
    if !cfg!(windows) {
        if let Ok(pid) = get_tmux_pid(&state.config.mc_tmux_session).await {
            memory_usage = get_memory_usage(&pid).await.unwrap();
        }
    }

    HtmlTemplate(StatusBoxTemplate {
        state: *state.server_state.lock().unwrap(),
        uptime,
        port: state.config.mc_port.clone(),
        player_count,
        memory_usage,
    })
}

fn format_uptime(timestamp: Option<Instant>) -> String {
    match timestamp {
        Some(value) => {
            let duration = value.elapsed();
            let minutes = (duration.as_secs() / 60) % 60;
            let hours = (duration.as_secs() / 60) / 60;

            format!("{hours}h {minutes}min")
        }
        None => "-".to_string(),
    }
}

async fn get_player_count(state: &AppState) -> anyhow::Result<String> {
    let response = state.rcon_client.list_player().await?;

    let re = Regex::new(r"(?i)there are (\d+) of a max of (\d+) players online(?::\s*(.*))?")?;
    let caps = re.captures(&response).unwrap();

    // Player Counts
    let online: u32 = caps
        .get(1)
        .context("missing online player count")?
        .as_str()
        .parse()?;

    let max: u32 = caps
        .get(2)
        .context("missing max player count")?
        .as_str()
        .parse()?;

    let count = format!("{online} / {max}");
    anyhow::Ok(count)
}

async fn stop(State(state): State<AppState>, Form(data): Form<FormData>) -> impl IntoResponse {
    if !verify_secret(&data.secret, &state.config.secret_hash) {
        return (
            StatusCode::FORBIDDEN,
            [(header::CACHE_CONTROL, "no-store")],
            r#"<div class="error">Message: Invalid secret.</div>"#,
        );
    }

    if *state.server_state.lock().unwrap() == ServerState::Offline {
        info!("MC Server is not running...");
        return (
            StatusCode::OK,
            [(header::CACHE_CONTROL, "no-store")],
            r#"<div class="success">Message: Server is not running.</div>"#,
        );
    }

    let _script_path: PathBuf = PathBuf::from(&state.config.mc_stop_script);
    let result = state.rcon_client.stop_server().await;
    // TODO Add option to disable rcon and use script instead if set

    match result {
        Ok(_) => {
            info!("MC server stop command spawned");

            let mut guard = state.server_state.lock().unwrap();
            *guard = ServerState::Stopping;

            (
                StatusCode::OK,
                [(header::CACHE_CONTROL, "no-store")],
                r#"<div class="success">Message: Server stop requested.</div>"#,
            )
        }
        Err(err) => {
            error!("Failed to execute command: {err:#}");

            (
                StatusCode::INTERNAL_SERVER_ERROR,
                [(header::CACHE_CONTROL, "no-store")],
                r#"<div class="error">Message: Failed to stop server.</div>"#,
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
        }
    };

    argon2::Argon2::default()
        .verify_password(provided_secret.as_bytes(), &parsed_hash)
        .is_ok()
}
