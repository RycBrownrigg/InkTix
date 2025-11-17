#!/bin/bash
# Script to fix HTTPS issues - creates SSL cert if missing and updates Nginx config

echo "=== Fixing HTTPS Configuration ==="
echo ""

# Check if SSL certificates exist
echo "1. Checking SSL certificates..."
ssh ryc@135.148.61.99 "if [ ! -f /etc/ssl/certs/inktix.crt ] || [ ! -f /etc/ssl/private/inktix.key ]; then
    echo 'SSL certificates not found. Creating self-signed certificate...'
    sudo openssl req -x509 -nodes -days 365 -newkey rsa:2048 \
        -keyout /etc/ssl/private/inktix.key \
        -out /etc/ssl/certs/inktix.crt \
        -subj '/C=US/ST=State/L=City/O=Organization/CN=135.148.61.99'
    sudo chmod 600 /etc/ssl/private/inktix.key
    sudo chmod 644 /etc/ssl/certs/inktix.crt
    echo 'SSL certificate created successfully'
else
    echo 'SSL certificates already exist'
fi"

echo ""
echo "2. Verifying Nginx configuration..."
ssh ryc@135.148.61.99 "sudo nginx -t"

echo ""
echo "3. Reloading Nginx..."
ssh ryc@135.148.61.99 "sudo systemctl reload nginx"

echo ""
echo "=== Done ==="
echo "Try accessing https://135.148.61.99 now"
echo "Note: You may see a browser warning about self-signed certificate - this is normal"

