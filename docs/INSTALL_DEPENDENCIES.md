# Installing Dependencies

## Quick Install Commands

### macOS (using Homebrew)

```bash
# Install Homebrew if not already installed
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install all dependencies
brew install rust postgresql@15 redis

# Start services
brew services start postgresql@15
brew services start redis

# Install Solana (optional for blockchain features)
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"

# Install Anchor (optional for smart contracts)
cargo install --git https://github.com/coral-xyz/anchor anchor-cli --locked
```

### Ubuntu/Debian

```bash
# Update package list
sudo apt-get update

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Install PostgreSQL
sudo apt-get install -y postgresql postgresql-contrib
sudo systemctl start postgresql
sudo systemctl enable postgresql

# Install Redis
sudo apt-get install -y redis-server
sudo systemctl start redis
sudo systemctl enable redis

# Install build dependencies
sudo apt-get install -y build-essential pkg-config libssl-dev

# Install Solana (optional)
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"

# Install Anchor (optional)
cargo install --git https://github.com/coral-xyz/anchor anchor-cli --locked
```

### Using Docker (Recommended for Development)

```bash
# Install Docker Desktop
# macOS: Download from https://www.docker.com/products/docker-desktop
# Linux: Follow https://docs.docker.com/engine/install/

# Start PostgreSQL and Redis with Docker Compose
cd superapp-rust
docker-compose up -d postgres redis

# Check services are running
docker-compose ps
```

## Minimal Setup (Without Redis)

If you want to run without Redis (not recommended for production):

```bash
# Install only Rust and PostgreSQL
brew install rust postgresql@15  # macOS
# or
sudo apt-get install rust postgresql  # Ubuntu

# Start PostgreSQL
brew services start postgresql@15  # macOS
# or
sudo systemctl start postgresql  # Ubuntu
```

Then update your `.env`:
```bash
# Comment out or remove Redis URL
# REDIS_URL=redis://localhost:6379
```

## Verification

Check all dependencies are installed:

```bash
# Rust
cargo --version
rustc --version

# PostgreSQL
psql --version
pg_isready

# Redis (optional)
redis-cli --version
redis-cli ping  # Should return PONG

# Solana (optional)
solana --version

# Anchor (optional)
anchor --version
```

## Quick Docker Setup

If you don't want to install dependencies locally, use Docker:

```bash
# Start all services
docker-compose up -d

# Run API in Docker
docker-compose up api

# View logs
docker-compose logs -f

# Stop all services
docker-compose down
```

## Troubleshooting

### Redis Connection Issues

If Redis is not installed but you want caching:

```bash
# Option 1: Install Redis locally
brew install redis && brew services start redis  # macOS
sudo apt-get install redis && sudo systemctl start redis  # Linux

# Option 2: Use Docker Redis
docker run -d -p 6379:6379 --name redis redis:7-alpine

# Option 3: Disable Redis in code (modify config)
# Set REDIS_URL="" in .env
```

### PostgreSQL Issues

```bash
# Create database
createdb superapp

# Or with psql
psql -U postgres -c "CREATE DATABASE superapp;"

# Check connection
psql postgresql://localhost:5432/superapp -c "SELECT 1;"
```

### Build Errors

```bash
# Install OpenSSL development package
# macOS
brew install openssl

# Ubuntu
sudo apt-get install libssl-dev pkg-config

# Update linker paths
export PKG_CONFIG_PATH="/usr/local/opt/openssl/lib/pkgconfig"
```

## Production Setup

For production environments:

1. **Use managed services**:
   - PostgreSQL: AWS RDS, Google Cloud SQL, Supabase
   - Redis: AWS ElastiCache, Redis Cloud, Upstash

2. **Or setup with Docker Swarm/Kubernetes**:
   - See `k8s/` directory for manifests
   - Use Helm charts for easier deployment

3. **Monitoring**:
   - Install Prometheus, Grafana
   - Configure Sentry for error tracking
   - Set up log aggregation (ELK, Loki)

## Next Steps

After installing dependencies:

1. Run setup script: `./scripts/setup.sh`
2. Configure `.env` file
3. Start development: `cargo run -p app-api`
