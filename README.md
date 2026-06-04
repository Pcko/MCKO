# MCKO - Minecraft Control

MCKO is a small Rust web application for controlling a personal Minecraft server from a browser.

> MCKO is designed for personal use only. It is not hardened for public-facing deployments.

## Features

- Web interface for controlling a specific Minecraft server
- Simple dashboard with start and stop controls
- Configurable server directory and startup script
- Environment-based configuration

## Tech Stack

- Rust
- Axum
- Askama
- HTMX
- rcon 

## Requirements

- Rust toolchain installed on the host machine
- Java installed on the host machine 
- A Minecraft server folder and jar
- A server startup script, such as `start_server.sh`
- The included example scripts use tmux. You can replace them with scripts for Docker, systemd, PowerShell, batch files, or another server setup

## Installation

Clone the repository:

```bash
git clone https://github.com/Pcko/MCKO
cd MCKO
```
Create a `.env` file in the project root (use `.env.example` as reference):

Example:
```env
# MCKO Server
# SERVER_HOST=127.0.0.1
# SERVER_PORT=3000
SECRET_HASH='$argon2id$v=19$m=19456,t=2,p=1$replace-with-generated-hash'

# MC Server 
# MC_HOST=127.0.0.1
# MC_PORT=25565
MC_SERVER_DIR=/path/to/minecraft/server
MC_SERVER_JAR=server.jar
MC_JAVA_ARGS=-Xms2G -Xmx6G

# Scripts
MC_START_SCRIPT=./scripts/start_server.sh
MC_STOP_SCRIPT=./scripts/stop_server.sh
MC_TMUX_SESSION=mcko

# RCON 
RCON_ENABLED=true
# RCON_HOST=127.0.0.1
# RCON_PORT=25575
RCON_PASSWORD=your-rcon-password
```

Scripts can be `.bat`,`.sh` or `.cmd`

Secrets need to be hashed (use argon2 with these settings : `$argon2id$v=19$m=19456,t=2,p=1$<salt>$<hash>`)

MCKO offers a util to create these hashes:
```bash
cargo run --bin secret-hash <secret>
```

## Running the App

Start the application with:
```bash
cargo run setup
cargo run
```