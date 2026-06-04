use dotenv::dotenv;

#[derive(Clone)]
pub struct AppConfig {
    pub server_host: String,
    pub server_port: String,
    pub secret_hash: String,
    pub mc_host: String,
    pub mc_port: String,
    pub mc_server_dir: String,
    pub mc_server_jar: String,
    pub mc_start_script: String,
    pub mc_stop_script: String,
    pub mc_tmux_session: String,
    pub rcon_host: String,
    pub rcon_port: String,
    pub rcon_password: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl AppConfig {
    pub fn new() -> Self {
        dotenv().ok();

        AppConfig {
            server_host: dotenv::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            server_port: dotenv::var("SERVER_PORT").unwrap_or_else(|_| "3000".to_string()),
            secret_hash: dotenv::var("SECRET_HASH").expect("SECRET_HASH must be set"),
            mc_host: dotenv::var("MC_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            mc_port: dotenv::var("MC_PORT").unwrap_or_else(|_| "25565".to_string()),
            mc_server_dir: dotenv::var("MC_SERVER_DIR").expect("MC_SERVER_DIR must be set"),
            mc_server_jar: dotenv::var("MC_SERVER_JAR").expect("MC_SERVER_JAR must be set"),
            mc_start_script: dotenv::var("MC_START_SCRIPT").expect("MC_START_SCRIPT must be set"),
            mc_stop_script: dotenv::var("MC_STOP_SCRIPT").expect("MC_STOP_SCRIPT must be set"),
            mc_tmux_session: dotenv::var("MC_TMUX_SESSION").unwrap_or_else(|_| "mcko".to_string()),
            rcon_host: dotenv::var("RCON_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            rcon_port: dotenv::var("RCON_PORT").unwrap_or_else(|_| "25575".to_string()),
            rcon_password: dotenv::var("RCON_PASSWORD").expect("RCON_PASSWORD must be set"),
        }
    }
}
