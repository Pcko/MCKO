use crate::app_config::AppConfig;
use std::sync::{Arc, atomic::AtomicBool};

#[derive(Clone)]
pub struct AppState {
   pub config: Arc<AppConfig>,
   pub server_running: Arc<AtomicBool>
}