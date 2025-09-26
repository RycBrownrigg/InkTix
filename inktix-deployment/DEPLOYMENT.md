# InkTix Production Deployment Guide

# ===================================

## Server Information

- **Domain**: inktix.com (or your domain)
- **Server IP**: 135.148.61.99
- **OS**: Debian 13.1 (Trixie)
- **Environment**: Production

## Prerequisites

### System Requirements

- **RAM**: Minimum 4GB, Recommended 8GB+
- **Storage**: Minimum 50GB SSD
- **CPU**: 2+ cores
- **Network**: Stable internet connection

### Required Software Stack

- Node.js 18+ (LTS)
- Nginx (Web server)
- PM2 (Process manager)
- Git
- Build tools (for Rust compilation)
- SSL certificates (Let's Encrypt)

### Debian-Specific Notes

- This guide is optimized for Debian 13.1 (Bookworm)
- `software-properties-common` package is not needed (Ubuntu-specific)
- NodeSource repository should work, but nvm is provided as fallback
- All package names and commands are Debian-compatible

## Step 1: Server Preparation

### 1.1 Update System

```bash
sudo apt update && sudo apt upgrade -y
sudo apt install -y curl wget git unzip apt-transport-https ca-certificates gnupg lsb-release
```

### 1.2 Install Node.js 20 LTS

```bash
# Install Node.js 20 LTS
# Method 1: Using NodeSource repository (recommended)
curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
sudo apt-get install -y nodejs

# Method 2: Alternative - using Node Version Manager (nvm) if Method 1 fails
# curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
# source ~/.bashrc
# nvm install 20
# nvm use 20

# Verify installation
node --version  # Should be v20.x.x
npm --version   # Should be 10.x.x
```

### 1.3 Install PM2 Process Manager

```bash
sudo npm install -g pm2
pm2 --version
```

### 1.4 Install Nginx

```bash
sudo apt install -y nginx
sudo systemctl enable nginx
sudo systemctl start nginx
```

### 1.5 Install Rust and Cargo (for smart contract compilation)

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Install additional Rust components
rustup target add wasm32-unknown-unknown
rustup component add rust-src

# Install cargo-contract for ink! development
cargo install --locked --version 4.0.0-beta cargo-contract

# Verify installation
rustc --version
cargo --version
cargo contract --version
```

### 1.6 Install Additional Build Dependencies

```bash
# Install build essentials and additional tools
sudo apt install -y build-essential pkg-config libssl-dev libudev-dev

# Install additional tools for ink! development
sudo apt install -y clang llvm-dev
```

## Step 2: Project Setup

### 2.1 Create Application Directory

```bash
sudo mkdir -p /var/www/inktix.com
sudo chown -R $USER:$USER /var/www/inktix.com
cd /var/www/inktix.com
```

### 2.2 Clone and Setup Project

```bash
# Clone the repository (replace with your actual repo URL)
git clone https://github.com/yourusername/InkTix.git .

# Or if you're uploading files directly:
# scp -r /path/to/InkTix/* user@135.148.61.99:/var/www/inktix.com/
```

### 2.3 Build Smart Contracts

```bash
# Build all smart contracts
cd contracts/sports_broker
cargo contract build --release

cd ../concert_broker
cargo contract build --release

cd ../inktix_core
cargo contract build --release

cd ../..
```

### 2.4 Build Frontend

```bash
cd frontend
npm install
npm run build

# Copy built files to deployment directory
cp -r dist/* /var/www/inktix.com/
cd ..
```

## Step 3: SSL Certificate Setup

### 3.1 Install Certbot

```bash
sudo apt install -y certbot python3-certbot-nginx
```

### 3.2 Obtain SSL Certificate

```bash
# Replace 'yourdomain.com' with your actual domain
sudo certbot --nginx -d yourdomain.com -d www.yourdomain.com

# Or if you don't have a domain yet, you can use the IP address
# Note: This will give you a self-signed certificate
sudo openssl req -x509 -nodes -days 365 -newkey rsa:2048 \
    -keyout /etc/ssl/private/inktix.key \
    -out /etc/ssl/certs/inktix.crt \
    -subj "/C=US/ST=State/L=City/O=Organization/CN=135.148.61.99"
```

## Step 4: Nginx Configuration

### 4.1 Create Nginx Configuration

```bash
sudo nano /etc/nginx/sites-available/inktix.com
```

### 4.2 Nginx Configuration Content

```nginx
# InkTix Frontend - Nginx Configuration
# Updated for IP: 135.148.61.99

server {
    listen 80;
    listen 443 ssl http2;
    server_name 135.148.61.99 yourdomain.com www.yourdomain.com;

    # SSL configuration
    ssl_certificate /etc/ssl/certs/inktix.crt;
    ssl_certificate_key /etc/ssl/private/inktix.key;

    # Security headers
    add_header X-Frame-Options "DENY" always;
    add_header X-Content-Type-Options "nosniff" always;
    add_header Referrer-Policy "origin-when-cross-origin" always;
    add_header X-XSS-Protection "1; mode=block" always;
    add_header Strict-Transport-Security "max-age=31536000; includeSubDomains" always;

    # Root directory
    root /var/www/inktix.com;
    index index.html;

    # Gzip compression
    gzip on;
    gzip_vary on;
    gzip_min_length 1024;
    gzip_proxied expired no-cache no-store private auth;
    gzip_types
        text/plain
        text/css
        text/xml
        text/javascript
        application/javascript
        application/xml+rss
        application/json
        application/wasm;

    # Cache static assets
    location ~* \.(js|css|png|jpg|jpeg|gif|ico|svg|woff|woff2|ttf|eot|wasm)$ {
        expires 1y;
        add_header Cache-Control "public, immutable";
        try_files $uri =404;
    }

    # Handle .wasm files
    location ~* \.wasm$ {
        add_header Content-Type "application/wasm";
        expires 1y;
        add_header Cache-Control "public, immutable";
    }

    # Handle client-side routing
    location / {
        try_files $uri $uri/ /index.html;
    }

    # Force HTTPS (uncomment if using domain with SSL)
    # if ($scheme != "https") {
    #     return 301 https://$server_name$request_uri;
    # }
}
```

### 4.3 Enable Site

```bash
sudo ln -s /etc/nginx/sites-available/inktix.com /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl reload nginx
```

## Step 5: Environment Configuration

### 5.1 Create Production Environment File

```bash
nano /var/www/inktix.com/.env.production
```

### 5.2 Environment Variables

```bash
# Production Environment Variables
NODE_ENV=production
NEXT_PUBLIC_API_URL=https://135.148.61.99/api
NEXT_PUBLIC_WS_URL=wss://135.148.61.99/ws
NEXT_PUBLIC_RPC_URL=wss://westend-rpc.polkadot.io
NEXT_PUBLIC_HTTP_RPC_URL=https://westend-rpc.polkadot.io

# Contract Addresses (update with your deployed contracts)
NEXT_PUBLIC_SPORTS_BROKER_ADDRESS=5CR7KXVKZ8tuNh7u3xY7tekt6s6HF2ZpemytdGrH5bt1jFbk
NEXT_PUBLIC_CONCERT_BROKER_ADDRESS=5EQcT6gpuQtTYfpy3ygbBC5UF9Y8rnCKMvuJ3NC7pCgtej4y
NEXT_PUBLIC_INKTIX_CORE_ADDRESS=5FN2wJEWQXus8k3wZdQM8Q1bmDquawNNGH97kAFr4WETF8fE

# Security
JWT_SECRET=your-super-secure-jwt-secret-key-for-production
SESSION_SECRET=your-super-secure-session-secret-key-for-production

# Database (if using)
DATABASE_URL=postgresql://inktix_user:your_secure_password@localhost:5432/inktix_production

# Redis (if using)
REDIS_URL=redis://localhost:6379
```

## Step 6: Process Management with PM2

### 6.1 Create PM2 Ecosystem File

```bash
nano /var/www/inktix.com/ecosystem.config.js
```

### 6.2 PM2 Configuration

```javascript
module.exports = {
  apps: [
    {
      name: "inktix-frontend",
      script: "npm",
      args: "start",
      cwd: "/var/www/inktix.com",
      env: {
        NODE_ENV: "production",
        PORT: 3000,
      },
      instances: 1,
      exec_mode: "fork",
      watch: false,
      max_memory_restart: "1G",
      error_file: "/var/log/inktix/error.log",
      out_file: "/var/log/inktix/out.log",
      log_file: "/var/log/inktix/combined.log",
      time: true,
    },
  ],
};
```

### 6.3 Create Log Directory

```bash
sudo mkdir -p /var/log/inktix
sudo chown -R $USER:$USER /var/log/inktix
```

### 6.4 Start Application with PM2

```bash
cd /var/www/inktix.com
pm2 start ecosystem.config.js
pm2 save
pm2 startup
```

## Step 7: Firewall Configuration

### 7.1 Configure UFW Firewall

```bash
sudo ufw allow 22/tcp    # SSH
sudo ufw allow 80/tcp    # HTTP
sudo ufw allow 443/tcp   # HTTPS
sudo ufw enable
sudo ufw status
```

## Step 8: Monitoring and Maintenance

### 8.1 Health Check Script

```bash
nano /var/www/inktix.com/health-check.sh
```

```bash
#!/bin/bash
# Health check script for InkTix

echo "=== InkTix Health Check ==="
echo "Date: $(date)"
echo ""

# Check if PM2 processes are running
echo "PM2 Status:"
pm2 status
echo ""

# Check if Nginx is running
echo "Nginx Status:"
sudo systemctl status nginx --no-pager
echo ""

# Check disk space
echo "Disk Usage:"
df -h
echo ""

# Check memory usage
echo "Memory Usage:"
free -h
echo ""

# Check if the application is responding
echo "Application Response:"
curl -I http://135.148.61.99 || echo "Application not responding"
echo ""

echo "=== Health Check Complete ==="
```

```bash
chmod +x /var/www/inktix.com/health-check.sh
```

### 8.2 Log Monitoring

```bash
# View PM2 logs
pm2 logs inktix-frontend

# View Nginx logs
sudo tail -f /var/log/nginx/access.log
sudo tail -f /var/log/nginx/error.log

# View application logs
tail -f /var/log/inktix/combined.log
```

## Step 9: Backup Strategy

### 9.1 Create Backup Script

```bash
nano /usr/local/bin/backup-inktix.sh
```

```bash
#!/bin/bash
# Backup script for InkTix

BACKUP_DIR="/var/backups/inktix"
DATE=$(date +%Y%m%d_%H%M%S)
APP_DIR="/var/www/inktix.com"

# Create backup directory
mkdir -p $BACKUP_DIR

# Create application backup
tar -czf $BACKUP_DIR/inktix_app_$DATE.tar.gz -C $APP_DIR .

# Keep only last 7 days of backups
find $BACKUP_DIR -name "inktix_app_*.tar.gz" -mtime +7 -delete

echo "Backup completed: inktix_app_$DATE.tar.gz"
```

```bash
chmod +x /usr/local/bin/backup-inktix.sh
```

### 9.2 Setup Cron Job for Backups

```bash
# Add to crontab
crontab -e

# Add this line for daily backups at 2 AM
0 2 * * * /usr/local/bin/backup-inktix.sh
```

## Step 10: Security Hardening

### 10.1 Update SSH Configuration

```bash
sudo nano /etc/ssh/sshd_config
```

Add/update these settings:

```
PermitRootLogin no
PasswordAuthentication no
PubkeyAuthentication yes
Port 2222  # Change from default port 22
```

```bash
sudo systemctl restart ssh
```

### 10.2 Install Fail2Ban

```bash
sudo apt install -y fail2ban
sudo systemctl enable fail2ban
sudo systemctl start fail2ban
```

### 10.3 Setup Automatic Security Updates

```bash
sudo apt install -y unattended-upgrades
sudo dpkg-reconfigure -plow unattended-upgrades
```

## Step 11: Performance Optimization

### 11.1 Nginx Performance Tuning

```bash
sudo nano /etc/nginx/nginx.conf
```

Add these optimizations:

```nginx
worker_processes auto;
worker_connections 1024;

# Gzip compression
gzip on;
gzip_vary on;
gzip_min_length 1024;
gzip_proxied any;
gzip_comp_level 6;
gzip_types
    text/plain
    text/css
    text/xml
    text/javascript
    application/javascript
    application/xml+rss
    application/json;

# Caching
open_file_cache max=1000 inactive=20s;
open_file_cache_valid 30s;
open_file_cache_min_uses 2;
open_file_cache_errors on;
```

### 11.2 System Optimization

```bash
# Increase file descriptor limits
echo "* soft nofile 65535" | sudo tee -a /etc/security/limits.conf
echo "* hard nofile 65535" | sudo tee -a /etc/security/limits.conf

# Optimize kernel parameters
echo "net.core.somaxconn = 65535" | sudo tee -a /etc/sysctl.conf
echo "net.ipv4.tcp_max_syn_backlog = 65535" | sudo tee -a /etc/sysctl.conf
sudo sysctl -p
```

## Step 12: Deployment Verification

### 12.1 Test Deployment

```bash
# Check if the application is running
curl -I http://135.148.61.99

# Check SSL (if configured)
curl -I https://135.148.61.99

# Check PM2 status
pm2 status

# Check Nginx status
sudo systemctl status nginx
```

### 12.2 Performance Test

```bash
# Install Apache Bench for testing
sudo apt install -y apache2-utils

# Run basic load test
ab -n 1000 -c 10 http://135.148.61.99/
```

## Troubleshooting

### Common Issues

1. **502 Bad Gateway**

   - Check if PM2 process is running: `pm2 status`
   - Check application logs: `pm2 logs inktix-frontend`
   - Verify port configuration in Nginx

2. **SSL Certificate Issues**

   - Check certificate validity: `openssl x509 -in /etc/ssl/certs/inktix.crt -text -noout`
   - Verify Nginx SSL configuration: `sudo nginx -t`

3. **Application Not Starting**

   - Check Node.js version: `node --version`
   - Verify dependencies: `npm install`
   - Check environment variables: `cat .env.production`

4. **Permission Issues**
   - Fix ownership: `sudo chown -R $USER:$USER /var/www/inktix.com`
   - Check file permissions: `ls -la /var/www/inktix.com`

### Useful Commands

```bash
# Restart services
sudo systemctl restart nginx
pm2 restart inktix-frontend

# Check logs
pm2 logs inktix-frontend --lines 100
sudo journalctl -u nginx -f

# Monitor resources
htop
df -h
free -h

# Check network
netstat -tlnp
ss -tlnp
```

## Maintenance Schedule

### Daily

- Check application status: `pm2 status`
- Monitor logs for errors
- Check disk space: `df -h`

### Weekly

- Run health check script
- Review security logs
- Update system packages: `sudo apt update && sudo apt upgrade`

### Monthly

- Review and rotate logs
- Check SSL certificate expiration
- Update application dependencies
- Review backup integrity

## Support

For deployment issues:

1. Check the logs first
2. Verify all services are running
3. Test network connectivity
4. Review configuration files

## Next Steps

After successful deployment:

1. Configure your domain DNS to point to 135.148.61.99
2. Update SSL certificates for your domain
3. Set up monitoring and alerting
4. Configure CDN if needed
5. Set up automated deployments

## Deployment Success (2025-09-24)

 **Successfully deployed on Debian 13.1 VPS (135.148.61.99)**

### Final Working Configuration

- **Server**: Debian 13.1 (Trixie) VPS
- **IP Address**: 135.148.61.99
- **Node.js**: 20.x LTS (via NodeSource repository with nvm fallback)
- **PM2**: Latest version with `serve` package for static files
- **Nginx**: 1.26+ with HTTP/2 and optimized configuration
- **SSL**: Self-signed certificates with SAN (ready for Let's Encrypt)
- **Firewall**: UFW with proper port configuration
- **Application**: Fully functional HTTP/HTTPS access

### Key Fixes Applied

1. **Debian Compatibility**: Removed Ubuntu-specific packages
2. **Nginx Configuration**: Fixed `gzip_proxied` directive syntax and HTTP/2 support
3. **cargo-contract Installation**: Multiple fallback methods implemented
4. **SSL Setup**: Self-signed certificates with SAN for proper IP support
5. **PM2 Integration**: Seamless process management with `serve` package
6. **Frontend Handling**: Graceful handling of missing source directories
7. **HTTP/2 Support**: Fixed deprecated `http2` directive syntax
8. **Blockchain Network**: Configured for Westend AssetHub with contracts pallet support
9. **Dynamic Contract Methods**: Smart contract manager automatically detects contract type and shows appropriate methods

### Health Check Results

- **PM2 Status**: Online (64MB memory usage)
- **Nginx Status**: Active and running
- **SSL Certificate**: Valid with SAN for IP address
- **Application Response**: HTTP/2 200 OK
- **System Resources**: Healthy (4GB RAM, 146GB storage)

### Access URLs

- **HTTP**: http://135.148.61.99 ✅
- **HTTPS**: https://135.148.61.99 ✅ (with self-signed certificate warning)

---

**Note**: This deployment guide assumes you have root/sudo access to the server. Always test in a staging environment first before deploying to production.
