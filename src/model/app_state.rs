use crate::{app_config::AppConfig, model::rcon_client::RconClient};
use crate::model::server_state::ServerState;
use std::{sync::{Arc, Mutex, RwLock}, time::Instant};

#[derive(Clone)]
pub struct AppState {
   pub config: Arc<AppConfig>,
   pub server_state: Arc<Mutex<ServerState>>,
   pub started_at: Arc<RwLock<Option<Instant>>>,
   pub rcon_client: Arc<RconClient>
}