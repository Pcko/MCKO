use crate::app_config::AppConfig;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AppState {
   pub config: Arc<AppConfig>,
   pub server_state: Arc<Mutex<ServerState>>
}


#[derive(Debug, PartialEq, Eq, Clone, Copy,)]
pub enum ServerState {
   Running, Starting, Offline, Stopping, Error
}

impl ServerState {
   pub fn label(self) -> &'static str {
      match self {
         ServerState::Offline => "Offline",
         ServerState::Starting => "Starting",
         ServerState::Running => "Running",
         ServerState::Stopping => "Stopping",
         ServerState::Error => "Error",
      }
   }

   pub fn css_class(self) -> &'static str {
      match self {
         ServerState::Offline => "offline",
         ServerState::Starting => "starting",
         ServerState::Running => "online",
         ServerState::Stopping => "stopping",
         ServerState::Error => "error",
      }
   }
}