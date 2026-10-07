#!/usr/bin/env bash

set -euo pipefail

PREFIX="/usr"
SYSTEMD_DIR="/etc/systemd/system"
DBUS_DIR="/usr/share/dbus-1/system.d"

BINARY="target/release/rde-facelockd"
SERVICE="packaging/systemd/rde-facelockd.service"
DBUS_POLICY="packaging/dbus/org.rde.FaceLock.conf"

echo "Building RDE FaceLock..."
# Run via a user not as root to avoid permission issues with cargo.
if [[ "${EUID}" -eq 0 ]]; then
  sudo -u "$SUDO_USER" cargo build --release --package rde-facelockd
else
  cargo build --release --package rde-facelockd
fi

echo "Installing RDE FaceLock..."

# Require root.
if [[ "${EUID}" -ne 0 ]]; then
  echo "Error: this script must be run as root."
  echo "Run: sudo ./scripts/install.sh"
  exit 1
fi

# Check required files.
for file in "$BINARY" "$SERVICE" "$DBUS_POLICY"; do
  if [[ ! -f "$file" ]]; then
    echo "Error: required file not found: $file"
    exit 1
  fi
done

echo "Creating rde system user..."

if ! id rde &>/dev/null; then
  useradd \
    --system \
    --no-create-home \
    --shell /usr/bin/nologin \
    rde
fi

echo "Installing FaceLock binary..."

install -Dm755 \
  "$BINARY" \
  "$PREFIX/bin/rde-facelockd"

echo "Installing systemd service..."

install -Dm644 \
  "$SERVICE" \
  "$SYSTEMD_DIR/rde-facelockd.service"

echo "Installing D-Bus policy..."

install -Dm644 \
  "$DBUS_POLICY" \
  "$DBUS_DIR/org.rde.FaceLock.conf"

echo "Reloading systemd..."

systemctl daemon-reload

echo
echo "RDE FaceLock installed successfully."
echo
echo "Start:"
echo "  sudo systemctl start rde-facelockd.service"
echo
echo "Enable at boot:"
echo "  sudo systemctl enable rde-facelockd.service"
echo
echo "Start and enable:"
echo "  sudo systemctl enable --now rde-facelockd.service"
echo
echo "Status:"
echo "  systemctl status rde-facelockd.service"
