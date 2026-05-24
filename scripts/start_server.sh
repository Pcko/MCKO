#!/usr/bin/env bash
set -e

if [ -z "$MC_SERVER_DIR" ]; then
    echo "MC_SERVER_DIR is not set"
    exit 1
fi

if [ -z "$MC_SERVER_JAR" ]; then
    echo "MC_SERVER_JAR is not set"
    exit 1
fi

cd "$MC_SERVER_DIR"

java ${MC_JAVA_ARGS:-"-Xms2G -Xmx6G"} -jar "$MC_SERVER_JAR" nogui