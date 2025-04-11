#!/bin/bash

set -e

COMPOSE_URL="https://github.com/docker/compose/releases/latest/download/docker-compose-$(uname -s)-$(uname -m)"
DEST="/usr/local/bin/docker-compose"

curl -L "$COMPOSE_URL" -o "$DEST"
chmod +x "$DEST"

ln -s "$DEST" /usr/bin/docker-compose || true

echo "Docker Compose installed successfully."
