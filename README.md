# MCKO - Minecraft Control

MCKO is a small Rust web application for controlling a personal Minecraft server from a browser. It is intended for trusted users on a private network and is not designed to be exposed directly to the public internet.

> MCKO is designed for personal use only. It is not hardened for public-facing deployments.

## Features

- Web interface for controlling a specific Minecraft server
- Start, stop, and restart controls
- Simple private-network dashboard
- Configurable server directory and startup script
- Environment-based configuration

## Tech Stack

- Rust
- Axum
- Askama
- HTMX

## Requirements

- Rust toolchain installed on the host machine
- Java installed on the host machine 
- tmux instlled on the host machine
- A Minecraft server folder and jar
- A server startup script, such as `start_server.sh` and stop script such as `stop_server.sh` (or use default ones in `./scripts`)

## Installation

Clone the repository:

```bash
git clone <repository-url>
cd MCKO
```
Create a `.env` file in the project root:

Example:
```env
SERVER_PORT=3000
SECRET='your-secret-here'

MC_SERVER_DIR=C:\MinecraftServer
MC_SERVER_JAR=my_server.jar

MC_JAVA_ARGS=-Xms2G -Xmx6G
MC_START_SCRIPT=start_server.sh
MC_STOP_SCRIPT=stop_server.sh
```

Scripts can be `.bat`,`.sh` or `.cmd`
Secrets need to be hashed (use argon2 with these settings : `$argon2id$v=19$m=19456,t=2,p=1$<salt>$<hash>`)

## Running the App

Start the application with:
```bash
cargo run
```