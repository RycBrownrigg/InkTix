#!/bin/bash
# Script to diagnose HTTPS issues on VPS

echo "=== Checking Nginx Configuration ==="
echo ""
echo "1. Current Nginx config for inktix.com:"
echo "----------------------------------------"
ssh ryc@135.148.61.99 "sudo cat /etc/nginx/sites-available/inktix.com | head -20"
echo ""

echo "2. Checking if SSL certificates exist:"
echo "----------------------------------------"
ssh ryc@135.148.61.99 "ls -la /etc/ssl/certs/inktix.crt /etc/ssl/private/inktix.key 2>&1"
echo ""

echo "3. Checking Nginx error logs:"
echo "----------------------------------------"
ssh ryc@135.148.61.99 "sudo tail -20 /var/log/nginx/error.log"
echo ""

echo "4. Testing Nginx configuration:"
echo "----------------------------------------"
ssh ryc@135.148.61.99 "sudo nginx -t"
echo ""

echo "5. Checking what's listening on port 443:"
echo "----------------------------------------"
ssh ryc@135.148.61.99 "sudo ss -ltnp | grep :443"
echo ""

echo "6. Checking enabled Nginx sites:"
echo "----------------------------------------"
ssh ryc@135.148.61.99 "ls -la /etc/nginx/sites-enabled/"
echo ""

