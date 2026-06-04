#!/usr/bin/env sh
set -eu

need_sudo() {
    if [ "$(id -u)" -eq 0 ]; then
        echo ""
    else
        echo "sudo"
    fi
}

SUDO="$(need_sudo)"

echo "Installing MCKO runtime dependencies..."

if command -v apt-get >/dev/null 2>&1; then
    $SUDO apt-get update
    $SUDO apt-get install -y tmux openjdk-21-jre-headless

elif command -v dnf >/dev/null 2>&1; then
    $SUDO dnf install -y tmux java-21-openjdk-headless

elif command -v pacman >/dev/null 2>&1; then
    $SUDO pacman -Sy --needed tmux jre-openjdk

elif command -v zypper >/dev/null 2>&1; then
    $SUDO zypper install -y tmux java-21-openjdk

elif command -v brew >/dev/null 2>&1; then
    brew install tmux openjdk

else
    echo "Unsupported package manager."
    echo "Please install manually:"
    echo "  - tmux"
    echo "  - Java 21 or newer"
    exit 1
fi

echo "Done."
echo "Run ./scripts/check_deps.sh to verify the installation."