# InkTix VPS Deployment Requirements

# ==================================

## Server Specifications

- **IP Address**: 135.148.61.99
- **Operating System**: Debian 13.1 (Trixie)
- **Architecture**: x86_64

## Minimum System Requirements

- **RAM**: 4GB (8GB recommended)
- **Storage**: 50GB SSD (100GB recommended)
- **CPU**: 2 cores (4 cores recommended)
- **Network**: Stable internet connection with static IP

## Required Software Stack

### 1. System Packages

```bash
# Essential packages
curl wget git unzip apt-transport-https ca-certificates gnupg lsb-release

# Build tools
build-essential pkg-config libssl-dev libudev-dev clang llvm-dev

# Security tools
fail2ban ufw unattended-upgrades
```

### 2. Node.js Environment

- **Node.js**: 20.x LTS or higher
- **NPM**: 10.x or higher
- **PM2**: Latest version for process management

### 3. Web Server

- **Nginx**: 1.18+ (for serving static files and reverse proxy)

### 4. Rust Toolchain

- **Rust**: Latest stable version
- **Cargo**: Latest version
- **Targets**: wasm32-unknown-unknown
- **cargo-contract**: 4.0.0-beta (for ink! development)

### 5. SSL/TLS

- **Certbot**: For Let's Encrypt certificates
- **OpenSSL**: For self-signed certificates (fallback)

## Network Requirements

### Ports to Open

- **22/tcp**: SSH access
- **80/tcp**: HTTP traffic
- **443/tcp**: HTTPS traffic

### Firewall Configuration

```bash
sudo ufw allow 22/tcp    # SSH
sudo ufw allow 80/tcp    # HTTP
sudo ufw allow 443/tcp   # HTTPS
sudo ufw enable
```

## File System Requirements

### Directory Structure

```
/var/www/inktix.com/          # Application root
├── dist/                     # Built frontend files
├── contracts/                # Smart contract files
├── .env.production          # Environment variables
├── ecosystem.config.js      # PM2 configuration
└── health-check.sh          # Health monitoring script

/var/log/inktix/              # Application logs
├── error.log
├── out.log
└── combined.log

/var/backups/inktix/          # Backup storage
└── inktix_app_*.tar.gz

/etc/nginx/sites-available/   # Nginx configuration
└── inktix.com

/etc/ssl/certs/               # SSL certificates
└── inktix.crt

/etc/ssl/private/             # SSL private keys
└── inktix.key
```

### Permissions

- **Application directory**: `www-data:www-data` with `755` permissions
- **Log directory**: `www-data:www-data` with `755` permissions
- **SSL certificates**: `root:root` with `644` (cert) and `600` (key) permissions

## Environment Variables Required

### Production Environment

```bash
NODE_ENV=production
NEXT_PUBLIC_API_URL=https://135.148.61.99/api
NEXT_PUBLIC_WS_URL=wss://135.148.61.99/ws
NEXT_PUBLIC_RPC_URL=wss://westend-rpc.polkadot.io
NEXT_PUBLIC_HTTP_RPC_URL=https://westend-rpc.polkadot.io

# Contract Addresses (update with deployed contracts)
NEXT_PUBLIC_SPORTS_BROKER_ADDRESS=5CR7KXVKZ8tuNh7u3xY7tekt6s6HF2ZpemytdGrH5bt1jFbk
NEXT_PUBLIC_CONCERT_BROKER_ADDRESS=5EQcT6gpuQtTYfpy3ygbBC5UF9Y8rnCKMvuJ3NC7pCgtej4y
NEXT_PUBLIC_INKTIX_CORE_ADDRESS=5FN2wJEWQXus8k3wZdQM8Q1bmDquawNNGH97kAFr4WETF8fE

# Security (generate secure random strings)
JWT_SECRET=your-super-secure-jwt-secret-key
SESSION_SECRET=your-super-secure-session-secret-key
```

## Security Requirements

### 1. SSH Configuration

- Disable root login
- Use key-based authentication
- Change default SSH port (optional)
- Enable fail2ban

### 2. Firewall

- Enable UFW
- Allow only necessary ports
- Block all other traffic

### 3. SSL/TLS

- Obtain SSL certificate (Let's Encrypt preferred)
- Configure HTTPS redirect
- Use strong cipher suites

### 4. System Updates

- Enable automatic security updates
- Regular system maintenance

## Performance Requirements

### 1. Nginx Optimization

- Enable gzip compression
- Configure caching headers
- Optimize worker processes

### 2. System Optimization

- Increase file descriptor limits
- Optimize kernel parameters
- Configure swap if needed

### 3. Monitoring

- Set up log rotation
- Monitor disk space
- Monitor memory usage
- Set up alerts

## Backup Requirements

### 1. Application Backup

- Daily automated backups
- 7-day retention policy
- Compressed archives

### 2. Configuration Backup

- Nginx configuration
- SSL certificates
- Environment files

### 3. Log Backup

- Log rotation
- Compressed old logs
- Remote backup (optional)

## Domain Configuration (Optional)

### If using a domain:

1. **DNS Records**:

   - A record: `yourdomain.com` → `135.148.61.99`
   - A record: `www.yourdomain.com` → `135.148.61.99`

2. **SSL Certificate**:

   - Use Let's Encrypt with domain validation
   - Auto-renewal configuration

3. **Nginx Configuration**:
   - Update server_name to include domain
   - Configure HTTPS redirect

## Pre-Deployment Checklist

- [ ] Server has required specifications
- [ ] All required packages can be installed
- [ ] Network ports are accessible
- [ ] SSH access is configured
- [ ] Domain DNS is configured (if using domain)
- [ ] Backup strategy is planned
- [ ] Monitoring is set up
- [ ] Security measures are in place

## Post-Deployment Verification

- [x] Application is accessible via HTTP/HTTPS
- [x] SSL certificate is valid
- [x] PM2 processes are running
- [x] Nginx is serving files correctly
- [x] Logs are being generated
- [x] Health check script works
- [x] Backup script works
- [x] Firewall is configured
- [x] System updates are enabled

## Deployment Success (2025-09-24)

 **Successfully deployed on Debian 13.1 VPS (135.148.61.99)**

### Key Learnings

1. **Debian Compatibility**: Removed Ubuntu-specific `software-properties-common` package
2. **Node.js Installation**: NodeSource repository works well, nvm as reliable fallback
3. **cargo-contract**: Multiple installation methods needed for reliability
4. **Nginx Configuration**: Fixed `gzip_proxied` directive syntax for Debian
5. **SSL Setup**: Self-signed certificates with SAN work for initial deployment
6. **PM2 Integration**: Seamless process management with `serve` package
7. **Frontend Build**: Graceful handling of missing source directories
8. **HTTP/2**: Fixed deprecated `http2` directive syntax

### Final Working Configuration

- **OS**: Debian 13.1 (Trixie)
- **Node.js**: 20.x LTS via NodeSource
- **PM2**: Latest version with `serve` package for static files
- **Nginx**: 1.26+ with optimized configuration and HTTP/2
- **SSL**: Self-signed certificates with SAN (ready for Let's Encrypt)
- **Firewall**: UFW with proper port configuration
- **Application**: Fully functional HTTP/HTTPS access

## Troubleshooting Resources

### Log Locations

- Application logs: `/var/log/inktix/`
- Nginx logs: `/var/log/nginx/`
- System logs: `/var/log/syslog`

### Useful Commands

```bash
# Check application status
pm2 status
pm2 logs inktix-frontend

# Check Nginx status
sudo systemctl status nginx
sudo nginx -t

# Check system resources
htop
df -h
free -h

# Check network
netstat -tlnp
ss -tlnp
```

## Support Information

- **Documentation**: See DEPLOYMENT.md for detailed instructions
- **Scripts**: Use deploy.sh for automated deployment
- **Health Check**: Run health-check.sh for system status
- **Backup**: Use backup-inktix.sh for data backup

---

**Note**: This requirements document assumes a fresh Debian 13.1 installation. Adjust accordingly if the server has existing software or configurations.
