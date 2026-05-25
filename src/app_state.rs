use crate::app_config::AppConfig;
use std::sync::{Arc, atomic::AtomicBool};


// TODO implement ServerState into AppState instad of only server_running (for UX)
#[derive(PartialEq, Eq)]
pub enum ServerState {
   RUNNING, STARTING, OFFLINE
}

#[derive(Clone)]
pub struct AppState {
   pub config: Arc<AppConfig>,
   pub server_running: Arc<AtomicBool>
}