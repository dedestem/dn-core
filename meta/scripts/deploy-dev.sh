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

echo ""
mkdir -p /etc/dn-core/
cp -r files/* /etc/dn-core/
cp target/release/dn-core /etc/dn-core/dn-core
cd /etc/dn-core/

echo "Starting dn-core and tailing logs..."
(
  sudo ./dn-core &
  echo "Waiting for logs..."
  echo ""
  while [ ! -f logs/latest.log ]; do sleep 0.1; done
  echo "Viewing logs/latest.log"
  echo "Use CTRL+C to stop dn-core and logging!"
  echo ""
  tail -f logs/latest.log
)
