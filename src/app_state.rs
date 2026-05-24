use crate::app_config::AppConfig;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
   pub config: Arc<AppConfig>,
}


impl AppState {
    pub fn new(config: Arc<AppConfig>) -> Self {
        Self { config }
    }
}