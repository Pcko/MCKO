#!/usr/bin/env bash
set -e

if [ -z "$MC_RCON_PASSWORD" ]; then
    echo "MC_RCON_PASSWORD is not set"
    exit 1
fi

mcrcon -H "${MC_RCON_HOST:-127.0.0.1}" -P "${MC_RCON_PORT:-25575}" -p "$MC_RCON_PASSWORD" "stop"