#!/usr/bin/env bash
# deploy_inktix_com.sh — Deploy InkTix to inktix.com (Namecheap shared hosting)
set -euo pipefail

VPS_USER_HOST="inkturrw@68.65.122.140"
SSH_PORT="21098"
WEB_ROOT="public_html"

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

SSH_OPTS="-p ${SSH_PORT}"
SCP_OPTS="-P ${SSH_PORT}"

echo "════════════════════════════════════════"
echo " InkTix Deploy → inktix.com"
echo "════════════════════════════════════════"

echo ""
echo "[1/5] Building frontend (Next.js static export)..."
cd frontend
npm run build
cd "$REPO_ROOT"

echo ""
echo "[2/5] Preparing remote temp directory..."
ssh ${SSH_OPTS} "${VPS_USER_HOST}" "rm -rf ~/inktix-deploy-temp && mkdir -p ~/inktix-deploy-temp"

echo ""
echo "[3/5] Uploading build output..."
scp ${SCP_OPTS} -qr frontend/out/* "${VPS_USER_HOST}":~/inktix-deploy-temp/

# Upload docs if present
[ -d docs ] && scp ${SCP_OPTS} -qr docs/ "${VPS_USER_HOST}":~/inktix-deploy-temp/docs/ 2>/dev/null || true

echo ""
echo "[4/5] Deploying with atomic swap..."
ssh ${SSH_OPTS} "${VPS_USER_HOST}" "
set -e
TARGET=\"\${HOME}/${WEB_ROOT}\"
TEMP=\"\${HOME}/inktix-deploy-temp\"

# Backup current version
rm -rf \"\${TARGET}.old\" 2>/dev/null || true

# Move current to .old (keep as backup)
if [ -d \"\${TARGET}\" ] && [ \"\$(ls -A \${TARGET} 2>/dev/null)\" ]; then
  mkdir -p \"\${TARGET}.old\"
  cp -a \"\${TARGET}/\"* \"\${TARGET}.old/\" 2>/dev/null || true
fi

# Deploy new files
cp -a \"\${TEMP}/\"* \"\${TARGET}/\"

# Add .htaccess for clean URLs (Apache/LiteSpeed on shared hosting)
cat > \"\${TARGET}/.htaccess\" << 'HTACCESS'
RewriteEngine On

# Force HTTPS
RewriteCond %{HTTPS} off
RewriteRule ^(.*)$ https://%{HTTP_HOST}%{REQUEST_URI} [L,R=301]

# Handle trailing slashes and clean URLs for Next.js static export
RewriteCond %{REQUEST_FILENAME} !-f
RewriteCond %{REQUEST_FILENAME} !-d
RewriteCond %{REQUEST_FILENAME}/index.html -f
RewriteRule ^(.*)$ /\$1/index.html [L]

# Custom 404
ErrorDocument 404 /404.html

# Cache static assets
<IfModule mod_expires.c>
  ExpiresActive On
  ExpiresByType text/css \"access plus 30 days\"
  ExpiresByType application/javascript \"access plus 30 days\"
  ExpiresByType image/png \"access plus 30 days\"
  ExpiresByType image/jpeg \"access plus 30 days\"
  ExpiresByType image/svg+xml \"access plus 30 days\"
  ExpiresByType font/woff2 \"access plus 30 days\"
</IfModule>

# Gzip compression
<IfModule mod_deflate.c>
  AddOutputFilterByType DEFLATE text/html text/css application/javascript application/json text/xml
</IfModule>
HTACCESS

# Cleanup temp
rm -rf \"\${TEMP}\"
echo 'Deploy complete'
"

echo ""
echo "[5/5] Verifying..."
HTTP_STATUS=$(curl -s -o /dev/null -w "%{http_code}" "https://inktix.com" 2>/dev/null || echo "000")
if [ "$HTTP_STATUS" = "200" ] || [ "$HTTP_STATUS" = "301" ] || [ "$HTTP_STATUS" = "302" ]; then
  echo "Site responding (HTTP ${HTTP_STATUS})"
else
  echo "Warning: Site returned HTTP ${HTTP_STATUS} (may need DNS propagation or SSL setup)"
fi

echo ""
echo "════════════════════════════════════════"
echo " InkTix deployed to inktix.com!"
echo " → https://inktix.com"
echo ""
echo " If this is the first deploy, you may need to:"
echo " 1. Set up SSL in cPanel → SSL/TLS → AutoSSL"
echo " 2. Wait for DNS propagation if you just pointed the domain"
echo "════════════════════════════════════════"
