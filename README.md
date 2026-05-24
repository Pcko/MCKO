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

- Rust installed
- Java installed on the host machine
- A Minecraft server folder
- A server startup script, such as `start_server.sh` and stop script such as `stop_server.sh`
- Access to the machine running the Minecraft server

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
SECRET=change-me
MC_SERVER_DIR=C:\MinecraftServer
MC_START_SCRIPT=start_server.sh
MC_STOP_SCRIPT=stop_server.sh
```

Scripts can be `.bat`,`.sh` or `.cmd`
Secrets need to be hashed (use argon 2 )
## Running the App

Start the application with:
```bash
cargo run
```