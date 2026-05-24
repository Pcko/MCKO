pids=$(lsof -tiTCP:25565 -sTCP:LISTEN)

if [ -n "$pids" ]; then
    kill $pids
else
    echo "No process listening on port 25565"
fi