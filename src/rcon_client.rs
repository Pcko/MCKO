use anyhow::{Context, Result};
use rcon::Connection;

pub struct RconClient {
    address: String,
    password: String,
}

impl RconClient {
    pub fn new(host: &str, port: &str, password: String) -> Self {
        Self {
            address: format!("{host}:{port}"),
            password,
        }
    }

    pub async fn run_command(&self, command: &str) -> Result<String> {
        let mut conn = Connection::builder()
            .connect(&self.address, &self.password)
            .await
            .context("rcon connection couldn't be established!")?;

        let response = conn
            .cmd(command)
            .await
            .with_context(|| format!("failed to execute RCON command: {command}"))?;

        Ok(response)
    }

    pub async fn stop_server(&self) -> Result<String> {
        self.run_command("stop")
            .await
            .context("rcon failed to run stop command ")
    }

    pub async fn list_player(&self) -> Result<String> {
        self.run_command("list")
            .await
            .context("rcon failed to run list command ")
    }
}
