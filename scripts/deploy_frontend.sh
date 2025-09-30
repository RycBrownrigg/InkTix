#!/usr/bin/env bash
set -euo pipefail

# Config
VPS_USER_HOST="ryc@135.148.61.99"
WEB_ROOT="/var/www/inktix.com"

# Resolve repo root
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$REPO_ROOT"

echo "[1/5] Building frontend..."
cd frontend
npm run build
cd "$REPO_ROOT"

# Prepare remote tmp dirs
echo "[2/5] Preparing remote temp directories..."
ssh -tt "$VPS_USER_HOST" "mkdir -p ~/frontend ~/docs || true; rm -rf ~/frontend/* ~/docs/* 2>/dev/null || true"

# Sync build output and docs
echo "[3/5] Copying build output and docs to VPS..."
scp -r frontend/out/* "$VPS_USER_HOST":~/frontend/
if [ -f markdown-viewer.html ]; then
  scp markdown-viewer.html "$VPS_USER_HOST":~/
fi
if [ -d docs ]; then
  scp -r docs/* "$VPS_USER_HOST":~/docs/
fi

# Deploy on server with sudo
echo "[4/5] Deploying to $WEB_ROOT on VPS (sudo required)..."
ssh -tt "$VPS_USER_HOST" "sudo bash -lc '
set -e
rm -rf "$WEB_ROOT"/*
cp -r ~/frontend/* "$WEB_ROOT"/
[ -f ~/markdown-viewer.html ] && cp ~/markdown-viewer.html "$WEB_ROOT"/
mkdir -p "$WEB_ROOT"/docs
[ -d ~/docs ] && cp -r ~/docs/* "$WEB_ROOT"/docs/
chown -R www-data:www-data "$WEB_ROOT"
chmod -R 755 "$WEB_ROOT"
rm -rf ~/frontend ~/docs
pm2 restart inktix-frontend
'"

echo "[5/5] Done. Deployed frontend to $WEB_ROOT and restarted PM2."
