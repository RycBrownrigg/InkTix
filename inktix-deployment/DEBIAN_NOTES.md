# Debian-Specific Deployment Notes

# =================================

## Package Differences from Ubuntu

### ❌ Not Available on Debian

- `software-properties-common` - Ubuntu-specific package

### ✅ Debian Alternatives

#### 1. Adding External Repositories

Instead of `software-properties-common`, use:

```bash
# For NodeSource repository
curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -

# For other repositories, add manually to sources.list
echo "deb https://deb.nodesource.com/node_20.x bookworm main" | sudo tee /etc/apt/sources.list.d/nodesource.list
```

#### 2. Alternative Node.js Installation Methods

**Method 1: NodeSource Repository (Recommended)**

```bash
curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
sudo apt-get install -y nodejs
```

**Method 2: Node Version Manager (nvm)**

```bash
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
source ~/.bashrc
nvm install 20
nvm use 20
```

**Method 3: Debian Backports (if available)**

```bash
echo "deb http://deb.debian.org/debian bookworm-backports main" | sudo tee /etc/apt/sources.list.d/backports.list
sudo apt update
sudo apt install -y nodejs/nodejs-backports
```

**Method 4: Snap Package (if snapd is installed)**

```bash
sudo snap install node --classic
```

## Debian 13.1 Specific Considerations

### 1. Package Manager

- Uses `apt` (not `apt-get` for most operations)
- `apt` is the modern frontend for `apt-get`

### 2. System Services

- Uses `systemctl` for service management
- Services are enabled with `systemctl enable`

### 3. Firewall

- UFW (Uncomplicated Firewall) works the same
- Alternative: `iptables` or `nftables`

### 4. SSL Certificates

- Let's Encrypt works the same with `certbot`
- Self-signed certificates use the same OpenSSL commands

## Troubleshooting Common Debian Issues

### 1. Node.js Installation Fails

```bash
# Check if curl is installed
sudo apt install -y curl

# Try alternative method
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
source ~/.bashrc
nvm install 20
```

### 2. Repository Issues

```bash
# Update package lists
sudo apt update

# Check for broken packages
sudo apt --fix-broken install

# Clean package cache
sudo apt clean
sudo apt autoremove
```

### 3. Permission Issues

```bash
# Ensure user is in sudo group
sudo usermod -aG sudo $USER

# Log out and back in, or run:
newgrp sudo
```

### 4. Service Not Starting

```bash
# Check service status
sudo systemctl status nginx
sudo systemctl status pm2-inktix

# Check logs
sudo journalctl -u nginx -f
pm2 logs inktix-frontend
```

## Debian-Optimized Installation Commands

### Complete System Setup

```bash
# Update system
sudo apt update && sudo apt upgrade -y

# Install essential packages (Debian-compatible)
sudo apt install -y \
    curl \
    wget \
    git \
    unzip \
    apt-transport-https \
    ca-certificates \
    gnupg \
    lsb-release \
    build-essential \
    pkg-config \
    libssl-dev \
    libudev-dev \
    clang \
    llvm-dev \
    fail2ban \
    ufw \
    unattended-upgrades

# Install Node.js (with fallback)
if curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -; then
    sudo apt-get install -y nodejs
else
    curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
    source ~/.bashrc
    nvm install 20
    nvm use 20
fi

# Install PM2
sudo npm install -g pm2

# Install Nginx
sudo apt install -y nginx

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
rustup target add wasm32-unknown-unknown
rustup component add rust-src
cargo install --locked --version 4.0.0-beta cargo-contract

# Install Certbot
sudo apt install -y certbot python3-certbot-nginx
```

## Verification Commands

### Check Installed Versions

```bash
# System info
lsb_release -a
uname -a

# Node.js
node --version
npm --version

# Rust
rustc --version
cargo --version
cargo contract --version

# Services
sudo systemctl status nginx
pm2 --version
```

### Test Network Connectivity

```bash
# Test external connectivity
curl -I https://google.com
ping -c 3 8.8.8.8

# Test local services
curl -I http://localhost
curl -I http://135.148.61.99
```

## Additional Debian Resources

- [Debian Package Search](https://packages.debian.org/)
- [Debian Administration Handbook](https://debian-handbook.info/)
- [Debian Wiki](https://wiki.debian.org/)
- [Node.js on Debian](https://nodejs.org/en/download/package-manager/#debian-and-ubuntu-based-linux-distributions)

## Deployment Success (2025-09-19)

✅ **Successfully deployed InkTix on Debian 13.1 VPS (135.148.61.99)**

### What Worked

1. **NodeSource Repository**: Primary method for Node.js installation
2. **nvm Fallback**: Reliable alternative when NodeSource fails
3. **cargo-contract**: Multiple installation methods ensure reliability
4. **Nginx Configuration**: Fixed syntax issues for Debian compatibility
5. **PM2 Integration**: Seamless process management
6. **UFW Firewall**: Simple and effective security setup

### Lessons Learned

1. **Package Differences**: Ubuntu-specific packages not available on Debian
2. **Repository Management**: Manual repository addition works better than `software-properties-common`
3. **Service Management**: `systemctl` works consistently across Debian versions
4. **SSL Setup**: Self-signed certificates work for initial deployment
5. **Process Management**: PM2 provides excellent process management and monitoring

### Recommended Approach

Use the automated `deploy.sh` script with the following modifications:
- Skip `software-properties-common` installation
- Use NodeSource repository with nvm fallback
- Implement multiple cargo-contract installation methods
- Fix Nginx configuration syntax for Debian

---

**Note**: This guide assumes you have a fresh Debian 13.1 installation. If you encounter issues, check the Debian documentation or community forums for specific solutions.

