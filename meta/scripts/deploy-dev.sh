#!/bin/bash
set -e

if [[ "$EUID" -ne 0 ]]; then
    echo "This script must be run with sudo or as root"
    exit 1
fi

if [[ -d "/etc/dn-core/" ]]; then
    echo "Deleting old /etc/dn-core/"
    sudo rm -rf /etc/dn-core/
fi

mkdir -p /etc/dn-core/
cp -r assets /etc/dn-core/assets
cp target/release/dn-core /etc/dn-core/dn-core
cd /etc/dn-core/

echo "Starting dn-core and tailing logs..."
(
  sudo ./dn-core &
  echo "Waiting for logs..."
  while [ ! -f logs/latest.log ]; do sleep 0.1; done
  echo "Viewing logs/latest.log"
  tail -f logs/latest.log
)
