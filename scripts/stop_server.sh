#!/usr/bin/env sh
set -eu

SESSION="${MC_TMUX_SESSION:-minecraft}"

if ! command -v tmux >/dev/null 2>&1; then
    echo "tmux is not installed"
    exit 1
fi

if ! tmux has-session -t "$SESSION" 2>/dev/null; then
    echo "Minecraft server is not running in tmux session: $SESSION"
    exit 0
fi

tmux send-keys -t "$SESSION" "stop" Enter

echo "Stop command sent to Minecraft server in tmux session: $SESSION"