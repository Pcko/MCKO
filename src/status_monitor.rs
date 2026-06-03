use std::time::Instant;
use std::{time::Duration};
use std::net::SocketAddr;

use tokio::net::{TcpStream};
use tracing::error;
use crate::model::server_state::ServerState;
use crate::model::app_state::{AppState};

pub fn spawn_status_monitor(state : AppState){
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(1));
        let mut last_state = false;
       
        loop {
            interval.tick().await;

            let running = is_server_running(&state.config.mc_host,&state.config.mc_port).await;
            let mut server_state = state.server_state.lock().unwrap();
            
            if running == last_state {
                continue;
            }

            if running {
                *server_state = ServerState::Running;
                *state.started_at.write().unwrap() = Some(Instant::now());
            } else {
                *server_state = ServerState::Offline;
                *state.started_at.write().unwrap() = None;
            }
            
            last_state = running;
        }
    });
}

async fn is_server_running(host: &str, port: &str) -> bool {
    let addr  : SocketAddr = match format!("{host}:{port}").parse() {
        Ok(addr) => addr,
        Err(err) => {
            error!("Invalid Minecraft server address: {err}");
            return false;
        }
    };

    match tokio::time::timeout(Duration::from_secs(1), TcpStream::connect(addr)).await {
        Ok(Ok(_stream)) => true,
        Ok(Err(_err)) => false,
        Err(_elapsed) => false,
    }
}