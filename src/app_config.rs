use dotenv::dotenv;

#[derive(Clone)]
pub struct AppConfig {
    pub server_port: String,
    pub secret_hash: String,
    pub mc_host : String, 
    pub mc_port: String,
    pub mc_server_dir: String,
    pub mc_start_script: String,
    pub mc_stop_script: String,
}

impl AppConfig {
    pub fn new() -> Self {
        dotenv().ok();
        
        AppConfig {
            secret_hash: dotenv::var("SECRET_HASH").expect("SECRET must be set"),
            server_port: dotenv::var("SERVER_PORT").unwrap_or_else(|_| "3000".to_string()),
            mc_host: dotenv::var("MC_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            mc_port: dotenv::var("MC_PORT").unwrap_or_else(|_| "25565".to_string()),
            mc_server_dir: dotenv::var("MC_SERVER_DIR").expect("MC_SERVER_DIR must be set"),
            mc_start_script: dotenv::var("MC_START_SCRIPT").expect("MC_START_SCRIPT must be set"),
            mc_stop_script: dotenv::var("MC_STOP_SCRIPT").expect("MC_STOP_SCRIPT must be set"),
        }
    }
}
