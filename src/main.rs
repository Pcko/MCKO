mod template;

use std::{env, net::{SocketAddr, TcpStream}, path::PathBuf, process::Command, time::Duration};

use dotenv::dotenv;
use axum::{Form, Router, http::{StatusCode, header}, response::IntoResponse, routing::{get, post}};
use serde::Deserialize;
use tower_http::services::ServeDir;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use template::{HtmlTemplate, DashboardTemplate}; 


#[tokio::main]
async fn main() {
    dotenv().ok();

    // logging for askama 
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();


    let app = app();
    // configure server
    let port = env::var("SERVER_PORT").unwrap();
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await.unwrap();
  
    info!("listening on {}", listener.local_addr().unwrap()); 
    axum::serve(listener, app).await.unwrap();
}

fn app() -> Router{
    // assets (tailwind css)
    let assets_path = std::env::current_dir().unwrap();

    Router::new()
        .route("/",get(dashboard))
        .route("/start", post(start))
        .route("/status", get(status))
        .nest_service("/assets",ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap())) )
}

async fn dashboard() -> impl IntoResponse{
    HtmlTemplate(DashboardTemplate{})
}

#[derive(Deserialize)]
struct FormData {
    secret: String,
}

async fn start(Form(data): Form<FormData>) -> impl IntoResponse {
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

    if is_server_running(){
        info!("MC Server already running...");
        return (
            StatusCode::FORBIDDEN,
            [(header::CACHE_CONTROL, "no-store")],
            r#"<div class="error">Already running.</div>"#,
        );
    }

    // Use the default script if env var is not set
    let mut command = Command::new("bash");
    let env_result = env::var("MC_START_SCRIPT");
    let server_dir = env::var("MC_SERVER_DIR").unwrap();

    if let Ok(script_name) = env_result {
        command.arg(format!("{server_dir}/{script_name}"));
    } else {
        let root = PathBuf::from(".");
        let script_path = root.join("scripts").join("start_server.sh");
        command.arg(script_path);
    }
    
    match command.spawn() {
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

async fn status() -> impl IntoResponse {

}

fn is_server_running() -> bool {
    let addr: SocketAddr = "127.0.0.1:25565"
        .parse()
        .expect("invalid address");

    TcpStream::connect_timeout(&addr, Duration::from_secs(1)).is_ok()
}