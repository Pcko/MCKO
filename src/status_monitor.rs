use std::{time::Duration};
use std::net::SocketAddr;

use tokio::net::{TcpStream};
use tracing::error;

use crate::app_state::{AppState, ServerState};

pub fn spawn_status_monitor(state : AppState){
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(1));

        loop {
            interval.tick().await;

            let running = is_server_running(&state.config.mc_port).await;
            let current_state;
        
            if running {
                current_state= ServerState::Running;
            }else {
                // could be a problem that while starting or stopping it will now always display offline
                current_state= ServerState::Offline;
            }

            let mut server_state= state.server_state.lock().unwrap();
            if *&server_state.eq(&current_state)  {
                *server_state = current_state;
            }
        }
    });
}

async fn is_server_running(port: &str) -> bool {
    let addr  : SocketAddr = match format!("127.0.0.1:{port}").parse() {
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