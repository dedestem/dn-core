#!/bin/bash
set -e

cargo build --release
sudo bash meta/scripts/deploy-dev.sh