#!/usr/bin/env bash
set -euo pipefail

# Config
VPS_USER_HOST="ryc@135.148.61.99"
WEB_ROOT="/var/www/inktix.com"
INKTIX_SUBDIR="$WEB_ROOT/inktix"

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
# Copy root index page and logo if they exist
if [ -f root-index.html ]; then
  scp root-index.html "$VPS_USER_HOST":~/
fi
if [ -f frontend/public/InkTix_logo.png ]; then
  scp frontend/public/InkTix_logo.png "$VPS_USER_HOST":~/InkTix_logo.png
fi

# Deploy on server with sudo
echo "[4/5] Deploying to $INKTIX_SUBDIR on VPS (sudo required)..."
ssh -tt "$VPS_USER_HOST" "sudo bash -lc '
set -e
HOME_DIR=\"/home/ryc\"
mkdir -p \"$INKTIX_SUBDIR\"
rm -rf \"$INKTIX_SUBDIR\"/*
cp -r \"\$HOME_DIR/frontend\"/* \"$INKTIX_SUBDIR\"/
[ -f \"\$HOME_DIR/markdown-viewer.html\" ] && cp \"\$HOME_DIR/markdown-viewer.html\" \"$INKTIX_SUBDIR\"/
mkdir -p \"$INKTIX_SUBDIR\"/docs
[ -d \"\$HOME_DIR/docs\" ] && cp -r \"\$HOME_DIR/docs\"/* \"$INKTIX_SUBDIR\"/docs/
# Copy root index page and logo if they exist
[ -f \"\$HOME_DIR/root-index.html\" ] && cp \"\$HOME_DIR/root-index.html\" \"$WEB_ROOT\"/index.html
[ -f \"\$HOME_DIR/InkTix_logo.png\" ] && cp \"\$HOME_DIR/InkTix_logo.png\" \"$WEB_ROOT\"/InkTix_logo.png
chown -R www-data:www-data \"$INKTIX_SUBDIR\"
chown -R www-data:www-data \"$WEB_ROOT\"/index.html 2>/dev/null || true
chown -R www-data:www-data \"$WEB_ROOT\"/InkTix_logo.png 2>/dev/null || true
chmod -R 755 \"$INKTIX_SUBDIR\"
chmod 644 \"$WEB_ROOT\"/index.html 2>/dev/null || true
chmod 644 \"$WEB_ROOT\"/InkTix_logo.png 2>/dev/null || true
rm -rf \"\$HOME_DIR/frontend\" \"\$HOME_DIR/docs\" \"\$HOME_DIR/root-index.html\" \"\$HOME_DIR/InkTix_logo.png\"
# Restart PM2 process if it exists (optional for static sites)
pm2 restart inktix-frontend 2>/dev/null || echo \"PM2 process not found (this is normal for static sites)\"
'"

echo "[5/5] Done. Deployed frontend to $INKTIX_SUBDIR."
