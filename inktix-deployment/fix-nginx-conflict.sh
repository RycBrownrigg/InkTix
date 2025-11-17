#!/bin/bash
# Script to fix Nginx conflicting server names and update to subdirectory config

echo "=== Fixing Nginx Configuration Conflicts ==="
echo ""

echo "1. Finding all Nginx configs with conflicting server_name..."
ssh ryc@135.148.61.99 "sudo grep -r 'server_name.*135.148.61.99' /etc/nginx/sites-available/ /etc/nginx/sites-enabled/ 2>/dev/null | grep -v '^#'"

echo ""
echo "2. Listing all enabled sites..."
ssh ryc@135.148.61.99 "ls -la /etc/nginx/sites-enabled/"

echo ""
echo "3. Checking default config..."
ssh ryc@135.148.61.99 "if [ -f /etc/nginx/sites-enabled/default ]; then
    echo 'Default config found - checking if it conflicts:'
    sudo grep -A 5 'server_name' /etc/nginx/sites-enabled/default | head -10
fi"

echo ""
echo "=== Next Steps ==="
echo "1. Disable conflicting configs (usually default)"
echo "2. Update inktix.com config with new subdirectory setup"
echo "3. Reload Nginx"

