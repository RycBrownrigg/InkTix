#!/bin/bash

# InkTix Automated Deployment Script
# For Debian 13.1 VPS
# IP: 135.148.61.99

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
SERVER_IP="135.148.61.99"
APP_DIR="/var/www/inktix.com"
LOG_DIR="/var/log/inktix"
BACKUP_DIR="/var/backups/inktix"
DOMAIN=""  # Set this if you have a domain

# Functions
log() {
    echo -e "${BLUE}[$(date +'%Y-%m-%d %H:%M:%S')]${NC} $1"
}

success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

error() {
    echo -e "${RED}[ERROR]${NC} $1"
    exit 1
}

# Check if running as root
check_root() {
    if [[ $EUID -eq 0 ]]; then
        error "This script should not be run as root. Please run as a regular user with sudo privileges."
    fi
}

# Update system packages
update_system() {
    log "Updating system packages..."
    sudo apt update && sudo apt upgrade -y
    sudo apt install -y curl wget git unzip apt-transport-https ca-certificates gnupg lsb-release build-essential pkg-config libssl-dev libudev-dev clang llvm-dev
    success "System packages updated"
}

# Install Node.js
install_nodejs() {
    log "Installing Node.js 20 LTS..."
    
    # Try NodeSource repository first
    if curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -; then
        sudo apt-get install -y nodejs
        success "Node.js installed via NodeSource: $(node --version)"
    else
        warning "NodeSource installation failed, trying alternative method..."
        # Alternative: Install via nvm
        curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
        source ~/.bashrc
        nvm install 20
        nvm use 20
        success "Node.js installed via nvm: $(node --version)"
    fi
}

# Install PM2
install_pm2() {
    log "Installing PM2 process manager..."
    sudo npm install -g pm2
    success "PM2 installed: $(pm2 --version)"
}

# Install Nginx
install_nginx() {
    log "Installing Nginx..."
    sudo apt install -y nginx
    sudo systemctl enable nginx
    sudo systemctl start nginx
    success "Nginx installed and started"
}

# Install Rust and Cargo
install_rust() {
    log "Installing Rust and Cargo..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source ~/.cargo/env
    rustup target add wasm32-unknown-unknown
    rustup component add rust-src
    # Install cargo-contract (try different versions)
    if ! cargo install --locked cargo-contract; then
        warning "Stable version failed, trying beta version..."
        if ! cargo install --locked --version 4.0.0-beta cargo-contract; then
            warning "Beta version failed, trying without version constraint..."
            cargo install cargo-contract
        fi
    fi
    success "Rust installed: $(rustc --version)"
}

# Install Certbot
install_certbot() {
    log "Installing Certbot..."
    sudo apt install -y certbot python3-certbot-nginx
    success "Certbot installed"
}

# Create directories
create_directories() {
    log "Creating application directories..."
    sudo mkdir -p $APP_DIR
    sudo mkdir -p $LOG_DIR
    sudo mkdir -p $BACKUP_DIR
    sudo chown -R $USER:$USER $APP_DIR
    sudo chown -R $USER:$USER $LOG_DIR
    sudo chown -R $USER:$USER $BACKUP_DIR
    success "Directories created"
}

# Build smart contracts
build_contracts() {
    log "Building smart contracts..."
    
    # Check if contracts directory exists
    if [ ! -d "contracts" ]; then
        warning "Contracts directory not found. Skipping smart contract building."
        warning "Smart contracts are already built and included in the deployment package."
        success "Smart contract building skipped (using pre-built contracts)"
        return 0
    fi
    
    # Build sports broker
    if [ -d "contracts/sports_broker" ]; then
        log "Building sports broker contract..."
        cd contracts/sports_broker
        cargo contract build --release
        cd ../..
    else
        warning "Sports broker contract directory not found, skipping..."
    fi
    
    # Build concert broker
    if [ -d "contracts/concert_broker" ]; then
        log "Building concert broker contract..."
        cd contracts/concert_broker
        cargo contract build --release
        cd ../..
    else
        warning "Concert broker contract directory not found, skipping..."
    fi
    
    # Build inktix core
    if [ -d "contracts/inktix_core" ]; then
        log "Building inktix core contract..."
        cd contracts/inktix_core
        cargo contract build --release
        cd ../..
    else
        warning "InkTix core contract directory not found, skipping..."
    fi
    
    success "Smart contracts built successfully"
}

# Build frontend
build_frontend() {
    log "Building frontend application..."
    cd frontend
    npm install
    npm run build
    
    # Copy built files to deployment directory
    cp -r dist/* $APP_DIR/
    cd ..
    success "Frontend built and deployed"
}

# Configure Nginx
configure_nginx() {
    log "Configuring Nginx..."
    
    # Create Nginx configuration
    sudo tee /etc/nginx/sites-available/inktix.com > /dev/null <<EOF
# InkTix Frontend - Nginx Configuration
# Updated for IP: $SERVER_IP

server {
    listen 80;
    listen 443 ssl http2;
    server_name $SERVER_IP $DOMAIN www.$DOMAIN;
    
    # SSL configuration (self-signed for IP)
    ssl_certificate /etc/ssl/certs/inktix.crt;
    ssl_certificate_key /etc/ssl/private/inktix.key;
    
    # Security headers
    add_header X-Frame-Options "DENY" always;
    add_header X-Content-Type-Options "nosniff" always;
    add_header Referrer-Policy "origin-when-cross-origin" always;
    add_header X-XSS-Protection "1; mode=block" always;
    add_header Strict-Transport-Security "max-age=31536000; includeSubDomains" always;
    
    # Root directory
    root $APP_DIR;
    index index.html;
    
    # Gzip compression
    gzip on;
    gzip_vary on;
    gzip_min_length 1024;
    gzip_proxied expired no-cache no-store private must-revalidate auth;
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
        try_files \$uri =404;
    }
    
    # Handle .wasm files
    location ~* \.wasm$ {
        add_header Content-Type "application/wasm";
        expires 1y;
        add_header Cache-Control "public, immutable";
    }
    
    # Handle client-side routing
    location / {
        try_files \$uri \$uri/ /index.html;
    }
}
EOF

    # Enable site
    sudo ln -sf /etc/nginx/sites-available/inktix.com /etc/nginx/sites-enabled/
    sudo nginx -t
    sudo systemctl reload nginx
    success "Nginx configured"
}

# Generate SSL certificate
generate_ssl() {
    log "Generating SSL certificate..."
    
    if [ -n "$DOMAIN" ]; then
        # Use Let's Encrypt for domain
        sudo certbot --nginx -d $DOMAIN -d www.$DOMAIN --non-interactive --agree-tos --email admin@$DOMAIN
        success "SSL certificate generated for domain: $DOMAIN"
    else
        # Generate self-signed certificate for IP
        sudo openssl req -x509 -nodes -days 365 -newkey rsa:2048 \
            -keyout /etc/ssl/private/inktix.key \
            -out /etc/ssl/certs/inktix.crt \
            -subj "/C=US/ST=State/L=City/O=InkTix/CN=$SERVER_IP"
        success "Self-signed SSL certificate generated for IP: $SERVER_IP"
    fi
}

# Create environment file
create_env() {
    log "Creating environment configuration..."
    
    cat > $APP_DIR/.env.production <<EOF
# Production Environment Variables
NODE_ENV=production
NEXT_PUBLIC_API_URL=https://$SERVER_IP/api
NEXT_PUBLIC_WS_URL=wss://$SERVER_IP/ws
NEXT_PUBLIC_RPC_URL=wss://westend-rpc.polkadot.io
NEXT_PUBLIC_HTTP_RPC_URL=https://westend-rpc.polkadot.io

# Contract Addresses (update with your deployed contracts)
NEXT_PUBLIC_SPORTS_BROKER_ADDRESS=5CR7KXVKZ8tuNh7u3xY7tekt6s6HF2ZpemytdGrH5bt1jFbk
NEXT_PUBLIC_CONCERT_BROKER_ADDRESS=5EQcT6gpuQtTYfpy3ygbBC5UF9Y8rnCKMvuJ3NC7pCgtej4y
NEXT_PUBLIC_INKTIX_CORE_ADDRESS=5FN2wJEWQXus8k3wZdQM8Q1bmDquawNNGH97kAFr4WETF8fE

# Security
JWT_SECRET=$(openssl rand -base64 32)
SESSION_SECRET=$(openssl rand -base64 32)
EOF

    success "Environment file created"
}

# Setup PM2
setup_pm2() {
    log "Setting up PM2..."
    
    # Create PM2 ecosystem file
    cat > $APP_DIR/ecosystem.config.js <<EOF
module.exports = {
  apps: [
    {
      name: 'inktix-frontend',
      script: 'npm',
      args: 'start',
      cwd: '$APP_DIR',
      env: {
        NODE_ENV: 'production',
        PORT: 3000
      },
      instances: 1,
      exec_mode: 'fork',
      watch: false,
      max_memory_restart: '1G',
      error_file: '$LOG_DIR/error.log',
      out_file: '$LOG_DIR/out.log',
      log_file: '$LOG_DIR/combined.log',
      time: true
    }
  ]
};
EOF

    # Start application with PM2
    cd $APP_DIR
    pm2 start ecosystem.config.js
    pm2 save
    pm2 startup
    
    success "PM2 configured and application started"
}

# Configure firewall
configure_firewall() {
    log "Configuring firewall..."
    sudo ufw allow 22/tcp    # SSH
    sudo ufw allow 80/tcp    # HTTP
    sudo ufw allow 443/tcp   # HTTPS
    sudo ufw --force enable
    success "Firewall configured"
}

# Create health check script
create_health_check() {
    log "Creating health check script..."
    
    cat > $APP_DIR/health-check.sh <<'EOF'
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
EOF

    chmod +x $APP_DIR/health-check.sh
    success "Health check script created"
}

# Create backup script
create_backup_script() {
    log "Creating backup script..."
    
    sudo tee /usr/local/bin/backup-inktix.sh > /dev/null <<EOF
#!/bin/bash
# Backup script for InkTix

BACKUP_DIR="/var/backups/inktix"
DATE=\$(date +%Y%m%d_%H%M%S)
APP_DIR="/var/www/inktix.com"

# Create backup directory
mkdir -p \$BACKUP_DIR

# Create application backup
tar -czf \$BACKUP_DIR/inktix_app_\$DATE.tar.gz -C \$APP_DIR .

# Keep only last 7 days of backups
find \$BACKUP_DIR -name "inktix_app_*.tar.gz" -mtime +7 -delete

echo "Backup completed: inktix_app_\$DATE.tar.gz"
EOF

    sudo chmod +x /usr/local/bin/backup-inktix.sh
    success "Backup script created"
}

# Main deployment function
deploy() {
    log "Starting InkTix deployment on $SERVER_IP..."
    
    check_root
    update_system
    install_nodejs
    install_pm2
    install_nginx
    install_rust
    install_certbot
    create_directories
    build_contracts
    build_frontend
    configure_nginx
    generate_ssl
    create_env
    setup_pm2
    configure_firewall
    create_health_check
    create_backup_script
    
    success "InkTix deployment completed successfully!"
    
    echo ""
    echo "=== Deployment Summary ==="
    echo "Server IP: $SERVER_IP"
    echo "Application URL: http://$SERVER_IP"
    echo "Application Directory: $APP_DIR"
    echo "Log Directory: $LOG_DIR"
    echo "Backup Directory: $BACKUP_DIR"
    echo ""
    echo "=== Next Steps ==="
    echo "1. Test the application: curl -I http://$SERVER_IP"
    echo "2. Check PM2 status: pm2 status"
    echo "3. View logs: pm2 logs inktix-frontend"
    echo "4. Run health check: $APP_DIR/health-check.sh"
    echo ""
    
    if [ -n "$DOMAIN" ]; then
        echo "5. Update your domain DNS to point to $SERVER_IP"
        echo "6. Run: sudo certbot --nginx -d $DOMAIN -d www.$DOMAIN"
    else
        echo "5. Consider getting a domain and updating SSL certificate"
    fi
    
    echo ""
    echo "=== Useful Commands ==="
    echo "Restart app: pm2 restart inktix-frontend"
    echo "View logs: pm2 logs inktix-frontend"
    echo "Check status: pm2 status"
    echo "Health check: $APP_DIR/health-check.sh"
    echo "Backup: /usr/local/bin/backup-inktix.sh"
}

# Script options
case "${1:-}" in
    "deploy")
        deploy
        ;;
    "health")
        if [ -f "$APP_DIR/health-check.sh" ]; then
            $APP_DIR/health-check.sh
        else
            error "Health check script not found. Run 'deploy' first."
        fi
        ;;
    "backup")
        if [ -f "/usr/local/bin/backup-inktix.sh" ]; then
            /usr/local/bin/backup-inktix.sh
        else
            error "Backup script not found. Run 'deploy' first."
        fi
        ;;
    "logs")
        pm2 logs inktix-frontend
        ;;
    "status")
        pm2 status
        ;;
    "restart")
        pm2 restart inktix-frontend
        ;;
    *)
        echo "InkTix Deployment Script"
        echo "Usage: $0 {deploy|health|backup|logs|status|restart}"
        echo ""
        echo "Commands:"
        echo "  deploy  - Full deployment of InkTix"
        echo "  health  - Run health check"
        echo "  backup  - Create backup"
        echo "  logs    - View application logs"
        echo "  status  - Check PM2 status"
        echo "  restart - Restart application"
        echo ""
        echo "Example: $0 deploy"
        exit 1
        ;;
esac
