mod template;

use std::env;

use dotenv::dotenv;
use axum::{Router, response::IntoResponse, routing::{get, post}};
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
                .unwrap_or_else(|_| "with_axum_htmx_askama=debug".into()),
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
        //.route("/start", post(|| "Start".into()))
        //.route("stop", post(|| "Stop".into()))
        .nest_service("/assets",ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap())) )
}

async fn dashboard() -> impl IntoResponse{
    HtmlTemplate(DashboardTemplate{})
}