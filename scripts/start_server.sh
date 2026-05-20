#!/bin/bash
set -e
set +x

if [ -z "$MC_SERVER_DIR" ]; then
    echo "MC_SERVER_DIR is not set"
    exit 1
fi

cd "$MC_SERVER_DIR"

java -Xms2G -Xmx6G -jar server.jar nogui