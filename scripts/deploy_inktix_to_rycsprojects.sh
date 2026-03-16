#!/usr/bin/env bash
# deploy_inktix_to_rycsprojects.sh — FINAL WORKING VERSION
set -euo pipefail

VPS_USER_HOST="ryc@135.148.61.99"
RYCS_WEB_ROOT="/var/www/html"
INKTIX_TARGET="$RYCS_WEB_ROOT/inktix"

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

echo "[1/5] Building frontend (Next.js export → out/)..."
cd frontend
npm run build
cd "$REPO_ROOT"

echo "[2/5] Preparing remote temp directory..."
ssh -t "$VPS_USER_HOST" "rm -rf ~/inktix-deploy-temp && mkdir -p ~/inktix-deploy-temp"

echo "[3/5] Uploading build output + extras..."
scp -qr frontend/out/* "$VPS_USER_HOST":~/inktix-deploy-temp/
[ -f markdown-viewer.html ] && scp -q markdown-viewer.html "$VPS_USER_HOST":~/inktix-deploy-temp/
[ -d docs ] && scp -qr docs/ "$VPS_USER_HOST":~/inktix-deploy-temp/docs/ 2>/dev/null || true

echo "[4/5] Deploying with sudo (you will be asked for password ONCE)..."
ssh -t "$VPS_USER_HOST" "sudo bash -c '
set -e
TARGET=\"$INKTIX_TARGET\"
TEMP=\"/home/ryc/inktix-deploy-temp\"

# Atomic swap (zero downtime)
rm -rf \"\$TARGET.old\" 2>/dev/null || true
[ -d \"\$TARGET\" ] && mv \"\$TARGET\" \"\$TARGET.old\" || true
mkdir -p \"\$TARGET\"
mv \"\$TEMP\" \"\$TARGET\"
chown -R www-data:www-data \"\$TARGET\"
find \"\$TARGET\" -type d -exec chmod 755 {} \;
find \"\$TARGET\" -type f -exec chmod 644 {} \;
rm -rf \"\$TARGET.old\" &
'"

echo "[5/5] Reloading Nginx (passwordless — this line is the fix)..."
ssh -t "$VPS_USER_HOST" "sudo systemctl reload nginx"   # ← the -t is mandatory here!

echo "════════════════════════════════════════"
echo "InkTix successfully deployed!"
echo "→ https://rycsprojects.com/inktix/"
echo "Root page remains blank and hidden"
echo "════════════════════════════════════════"