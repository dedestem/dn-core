#!/bin/bash
set -e

# Install dependencies
apt-get update
apt-get install -y ca-certificates curl gnupg lsb-release

# Set up Docker keyring
install -m 0755 -d /etc/apt/keyrings
curl -fsSL https://download.docker.com/linux/ubuntu/gpg -o /etc/apt/keyrings/docker.asc
chmod a+r /etc/apt/keyrings/docker.asc

# Get codename (e.g., bullseye, bookworm, focal, jammy)
CODENAME=$(lsb_release -cs)

# Add Docker's repository
echo \
  "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.asc] \
  https://download.docker.com/linux/ubuntu $CODENAME stable" | \
  tee /etc/apt/sources.list.d/docker.list > /dev/null

# Update apt with new repo
apt-get update

# Install Docker and plugins
apt-get install -y \
  docker-ce \
  docker-ce-cli \
  containerd.io \
  docker-buildx-plugin \
  docker-compose-plugin