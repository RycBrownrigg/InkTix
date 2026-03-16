#!/usr/bin/env bash
# deploy_inktix.sh — Deploy InkTix to inktix.com / inktix.rycsprojects.com
# Serves at root (no basePath) on both domains.
set -euo pipefail

VPS_USER_HOST="ryc@135.148.61.99"
INKTIX_TARGET="/var/www/projects/inktix/public"

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

echo "════════════════════════════════════════"
echo " InkTix Deploy → inktix.rycsprojects.com"
echo "════════════════════════════════════════"

echo ""
echo "[1/5] Building frontend (Next.js static export)..."
cd frontend
npm run build
cd "$REPO_ROOT"

echo ""
echo "[2/5] Preparing remote temp directory..."
ssh "$VPS_USER_HOST" "rm -rf ~/inktix-deploy-temp && mkdir -p ~/inktix-deploy-temp"

echo ""
echo "[3/5] Uploading build output..."
scp -qr frontend/out/* "$VPS_USER_HOST":~/inktix-deploy-temp/

# Upload docs if present
[ -d docs ] && scp -qr docs/ "$VPS_USER_HOST":~/inktix-deploy-temp/docs/ 2>/dev/null || true

echo ""
echo "[4/5] Deploying with atomic swap (zero downtime)..."
ssh -t "$VPS_USER_HOST" "sudo bash -c '
set -e
TARGET=\"$INKTIX_TARGET\"
TEMP=\"/home/ryc/inktix-deploy-temp\"

# Ensure parent dir exists
mkdir -p \"\$(dirname \$TARGET)\"

# Atomic swap
rm -rf \"\$TARGET.old\" 2>/dev/null || true
[ -d \"\$TARGET\" ] && mv \"\$TARGET\" \"\$TARGET.old\" || true
mv \"\$TEMP\" \"\$TARGET\"
chown -R www-data:www-data \"\$TARGET\"
find \"\$TARGET\" -type d -exec chmod 755 {} \;
find \"\$TARGET\" -type f -exec chmod 644 {} \;
rm -rf \"\$TARGET.old\" &
'"

echo ""
echo "[5/5] Reloading Nginx..."
ssh -t "$VPS_USER_HOST" "sudo systemctl reload nginx"

echo ""
echo "════════════════════════════════════════"
echo " InkTix deployed successfully!"
echo " → https://inktix.rycsprojects.com"
echo "════════════════════════════════════════"
