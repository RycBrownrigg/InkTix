# Moving InkTix to /inktix/ Subdirectory

This guide explains how to move the InkTix project from the root (`135.148.61.99/`) to a subdirectory (`135.148.61.99/inktix/`).

## Changes Made

### 1. Next.js Configuration
- **File**: `frontend/next.config.js`
- **Change**: Added `basePath: "/inktix"`
- This tells Next.js to prefix all routes with `/inktix`

### 2. Deployment Script
- **File**: `scripts/deploy_frontend.sh`
- **Change**: Updated to deploy to `/var/www/inktix.com/inktix/` instead of `/var/www/inktix.com/`

### 3. Nginx Configuration
- **File**: `inktix-deployment/nginx-inktix-subdirectory.conf`
- **New**: Created configuration file for serving from `/inktix/` subdirectory

## Deployment Steps

### Step 1: Update Nginx Configuration on VPS

SSH into your VPS and update the Nginx configuration:

```bash
ssh ryc@135.148.61.99

# Backup current config
sudo cp /etc/nginx/sites-available/inktix.com /etc/nginx/sites-available/inktix.com.backup

# Copy the new configuration
sudo nano /etc/nginx/sites-available/inktix.com
```

Then paste the contents from `inktix-deployment/nginx-inktix-subdirectory.conf` or copy it:

```bash
# From your local machine
scp inktix-deployment/nginx-inktix-subdirectory.conf ryc@135.148.61.99:~/

# On VPS
sudo cp ~/nginx-inktix-subdirectory.conf /etc/nginx/sites-available/inktix.com
```

Test and reload Nginx:

```bash
sudo nginx -t
sudo systemctl reload nginx
```

### Step 2: Deploy Frontend

From your local machine, run the deployment script:

```bash
./scripts/deploy_frontend.sh
```

This will:
- Build the frontend with `basePath: "/inktix"`
- Deploy files to `/var/www/inktix.com/inktix/` on the VPS

### Step 3: Update Contract Artifacts Path (if needed)

If you have contract artifacts served from `/contracts/`, you'll need to either:

**Option A**: Move them to `/inktix/contracts/`
```bash
# On VPS
sudo mkdir -p /var/www/inktix.com/inktix/contracts
sudo cp -r /var/www/inktix.com/contracts/* /var/www/inktix.com/inktix/contracts/
```

**Option B**: Update Nginx to serve `/contracts/` from the root (outside `/inktix/`)
```nginx
# Add to nginx config
location /contracts/ {
    alias /var/www/inktix.com/contracts/;
    try_files $uri =404;
}
```

### Step 4: Verify

1. Visit `https://135.148.61.99/inktix/` - should show the InkTix homepage
2. Test navigation - all internal links should work
3. Test contract artifacts - verify `/inktix/contracts/` paths work

## Important Notes

- **All internal links**: Next.js `Link` components automatically handle `basePath`, so no code changes needed
- **External URLs**: External URLs (like Unsplash images) are unaffected
- **API calls**: If you have API endpoints, they may need updating
- **Contract artifacts**: Make sure contract artifacts are accessible at the correct path

## Troubleshooting

### 404 Errors
- Check Nginx configuration is correct
- Verify files are in `/var/www/inktix.com/inktix/`
- Check file permissions: `sudo chown -R www-data:www-data /var/www/inktix.com/inktix`

### Assets Not Loading
- Clear browser cache
- Check browser console for 404 errors
- Verify `basePath` is set in `next.config.js`

### Contract Artifacts Not Found
- Check if artifacts are in the correct location
- Update Nginx config to serve from correct path
- Verify paths in frontend code match deployment structure

