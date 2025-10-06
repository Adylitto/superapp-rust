# Fly.io Deployment Guide

## 🚀 Quick Start

### Automated Deployment

```bash
cd /Users/damani/Code/superapp-rust
chmod +x scripts/deploy-flyio.sh
./scripts/deploy-flyio.sh
```

This script will:
1. Install/verify Fly CLI
2. Authenticate with Fly.io
3. Create app (if needed)
4. Set up PostgreSQL database
5. Configure secrets
6. Build and deploy

### Manual Deployment

#### 1. Install Fly CLI

```bash
curl -L https://fly.io/install.sh | sh

# Add to PATH (add to ~/.zshrc or ~/.bashrc)
export FLYCTL_INSTALL="/Users/damani/.fly"
export PATH="$FLYCTL_INSTALL/bin:$PATH"
```

#### 2. Login to Fly.io

```bash
flyctl auth login
```

#### 3. Create App

```bash
cd /Users/damani/Code/superapp-rust

# Launch will use fly.toml configuration
flyctl launch --no-deploy
```

Choose:
- App name: `superapp-rust-api`
- Region: `iad` (US East)
- PostgreSQL: Yes (creates database)
- Redis: Optional

#### 4. Create PostgreSQL Database

```bash
# Create database cluster
flyctl postgres create \
    --name superapp-db \
    --region iad \
    --initial-cluster-size 1 \
    --vm-size shared-cpu-1x \
    --volume-size 1

# Attach to app (sets DATABASE_URL automatically)
flyctl postgres attach superapp-db --app superapp-rust-api
```

#### 5. Set Environment Secrets

```bash
# Generate JWT secret
JWT_SECRET=$(openssl rand -base64 32)

# Set secrets
flyctl secrets set \
    JWT_SECRET="$JWT_SECRET" \
    RUST_LOG="info" \
    ALLOWED_ORIGINS="https://frontend-1gi1v4oel-adyls-projects-b070f026.vercel.app" \
    --app superapp-rust-api
```

#### 6. Deploy

```bash
flyctl deploy
```

## 📝 Configuration Files

### fly.toml

Main configuration file for Fly.io deployment:

```toml
app = "superapp-rust-api"
primary_region = "iad"

[build]
  dockerfile = "Dockerfile"

[env]
  RUST_LOG = "info"
  HOST = "0.0.0.0"
  PORT = "8080"

[http_service]
  internal_port = 8080
  force_https = true
```

### Dockerfile

Multi-stage build for optimal image size:

- **Builder stage**: Compiles Rust application
- **Runtime stage**: Minimal Debian with only runtime dependencies
- Final image size: ~100MB

### .dockerignore

Excludes unnecessary files from build context:
- Frontend files
- Solana smart contracts
- Documentation
- Development files

## 🗄️ Database Management

### Connect to PostgreSQL

```bash
# Via flyctl
flyctl postgres connect --app superapp-db

# Get connection string
flyctl postgres db list --app superapp-db
```

### Run Migrations

```bash
# SSH into app
flyctl ssh console --app superapp-rust-api

# Inside container
cd /app
# Run your migration tool (e.g., sqlx, diesel)
```

### Backup Database

```bash
flyctl postgres backup create --app superapp-db
flyctl postgres backup list --app superapp-db
```

## 🔍 Monitoring & Debugging

### View Logs

```bash
# Real-time logs
flyctl logs --app superapp-rust-api

# Follow logs
flyctl logs -f --app superapp-rust-api
```

### Check Status

```bash
flyctl status --app superapp-rust-api
```

### SSH Into Machine

```bash
flyctl ssh console --app superapp-rust-api
```

### Health Checks

The app includes a health check endpoint at `/health`:

```bash
curl https://superapp-rust-api.fly.dev/health
```

## 🔄 Updates & Rollbacks

### Deploy Updates

```bash
# Deploy new version
flyctl deploy

# Deploy with specific Dockerfile
flyctl deploy --dockerfile Dockerfile
```

### Rollback

```bash
# List releases
flyctl releases --app superapp-rust-api

# Rollback to previous version
flyctl releases rollback <version> --app superapp-rust-api
```

## 📊 Scaling

### Vertical Scaling (VM Size)

```bash
# View available VM sizes
flyctl platform vm-sizes

# Scale up
flyctl scale vm shared-cpu-2x --app superapp-rust-api
```

### Horizontal Scaling (Multiple Machines)

```bash
# Scale to 2 machines
flyctl scale count 2 --app superapp-rust-api

# Scale in specific regions
flyctl scale count 2 --region iad --app superapp-rust-api
flyctl scale count 1 --region lax --app superapp-rust-api
```

### Memory Scaling

```bash
flyctl scale memory 1024 --app superapp-rust-api
```

## 🔐 Secrets Management

### Set Secrets

```bash
flyctl secrets set KEY=value --app superapp-rust-api
```

### List Secrets

```bash
flyctl secrets list --app superapp-rust-api
```

### Remove Secrets

```bash
flyctl secrets unset KEY --app superapp-rust-api
```

### Import from .env

```bash
flyctl secrets import < .env.production --app superapp-rust-api
```

## 🌐 Custom Domains

### Add Custom Domain

```bash
# Add domain
flyctl certs add yourdomain.com --app superapp-rust-api

# Check certificate status
flyctl certs show yourdomain.com --app superapp-rust-api
```

### DNS Configuration

Add DNS records:
- **A Record**: `@` → Fly.io IP
- **AAAA Record**: `@` → Fly.io IPv6
- **CNAME**: `www` → `yourdomain.com`

Get IP addresses:
```bash
flyctl ips list --app superapp-rust-api
```

## 💰 Costs

### Free Tier Includes
- 3 shared-cpu-1x VMs
- 160GB outbound data transfer
- PostgreSQL with 3GB storage

### Estimated Costs (Beyond Free Tier)
- **shared-cpu-1x**: ~$0.0000022/s (~$5.70/month)
- **PostgreSQL**: ~$0.02/GB/month for storage
- **Outbound data**: $0.02/GB

### Monitor Costs

```bash
flyctl dashboard metrics --app superapp-rust-api
```

## 🚨 Troubleshooting

### Build Fails

```bash
# Check build logs
flyctl logs --app superapp-rust-api

# Test Docker build locally
docker build -t superapp-test .
docker run -p 8080:8080 superapp-test
```

### Database Connection Issues

```bash
# Check DATABASE_URL is set
flyctl secrets list --app superapp-rust-api

# Test database connection
flyctl postgres connect --app superapp-db
```

### App Not Starting

```bash
# Check health status
flyctl status --app superapp-rust-api

# View detailed logs
flyctl logs --app superapp-rust-api

# Restart app
flyctl apps restart superapp-rust-api
```

### Out of Memory

```bash
# Increase memory
flyctl scale memory 1024 --app superapp-rust-api

# Check memory usage
flyctl status --app superapp-rust-api
```

## 📋 Pre-Deployment Checklist

- [ ] Dockerfile tested locally
- [ ] Environment variables configured
- [ ] Database migrations ready
- [ ] JWT secret generated
- [ ] CORS origins configured
- [ ] Health check endpoint working
- [ ] Logging configured
- [ ] Error handling implemented
- [ ] Rate limiting configured (if needed)
- [ ] Security headers added

## 🔗 Useful Links

- **Fly.io Docs**: https://fly.io/docs
- **Rust on Fly**: https://fly.io/docs/languages-and-frameworks/rust/
- **PostgreSQL Docs**: https://fly.io/docs/postgres/
- **Dashboard**: https://fly.io/dashboard

## 📞 Support

- **Community**: https://community.fly.io
- **Status**: https://status.flyio.net
- **Email**: support@fly.io

---

**Last Updated**: 2025-10-06
