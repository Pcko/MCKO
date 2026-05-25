#!/usr/bin/env sh
set -eu

SESSION="${MC_TMUX_SESSION:-minecraft}"

if [ -z "${MC_SERVER_DIR:-}" ]; then
    echo "MC_SERVER_DIR is not set"
    exit 1
fi

if [ -z "${MC_SERVER_JAR:-}" ]; then
    echo "MC_SERVER_JAR is not set"
    exit 1
fi

if ! command -v tmux >/dev/null 2>&1; then
    echo "tmux is not installed"
    exit 1
fi

if ! command -v java >/dev/null 2>&1; then
    echo "java is not installed"
    exit 1
fi

if [ ! -d "$MC_SERVER_DIR" ]; then
    echo "MC_SERVER_DIR does not exist: $MC_SERVER_DIR"
    exit 1
fi

if [ ! -f "$MC_SERVER_DIR/$MC_SERVER_JAR" ]; then
    echo "MC_SERVER_JAR does not exist: $MC_SERVER_DIR/$MC_SERVER_JAR"
    exit 1
fi

if tmux has-session -t "$SESSION" 2>/dev/null; then
    echo "Minecraft server is already running in tmux session: $SESSION"
    exit 0
fi

cd "$MC_SERVER_DIR"

JAVA_ARGS="${MC_JAVA_ARGS:--Xms2G -Xmx6G}"

tmux new-session -d -s "$SESSION" \
    "java $JAVA_ARGS -jar '$MC_SERVER_JAR' nogui"

echo "Minecraft server started in tmux session: $SESSION"