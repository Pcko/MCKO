use std::{sync::atomic::Ordering, time::Duration};
use std::net::SocketAddr;

use tokio::net::{TcpStream};
use tracing::error;

use crate::app_state::AppState;

pub fn spawn_status_monitor(state : AppState){
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(1));

        loop {
            interval.tick().await;

            let running = is_server_running(&state.config.mc_port).await;

            state
                .server_running
                .store(running, Ordering::Relaxed);
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