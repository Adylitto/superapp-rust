# Quick Start Guide

Get SuperApp running in 5 minutes!

## Option 1: Full Docker Setup (Easiest)

```bash
cd superapp-rust

# Start all services including API
docker-compose up -d

# Check health
curl http://localhost:8080/health

# View logs
docker-compose logs -f api
```

That's it! Everything is running.

## Option 2: Docker Dependencies + Local API (Recommended for Development)

```bash
cd superapp-rust

# Start only PostgreSQL and Redis
docker-compose -f docker-compose.dev.yml up -d

# Setup environment
cp .env.example .env
# Edit .env if needed (defaults should work with Docker)

# Build and run API locally
cargo run -p app-api

# Check health
curl http://localhost:8080/health
```

## Option 3: All Local (Most Control)

### Step 1: Install Dependencies

```bash
# macOS
brew install rust postgresql@15 redis

# Ubuntu/Debian
sudo apt-get install rust postgresql redis-server

# Start services
brew services start postgresql@15 redis  # macOS
sudo systemctl start postgresql redis    # Linux
```

### Step 2: Setup Project

```bash
cd superapp-rust

# Run setup script
chmod +x scripts/setup.sh
./scripts/setup.sh

# Or manual setup
cargo build
cp .env.example .env
```

### Step 3: Run API

```bash
cargo run -p app-api

# In another terminal
curl http://localhost:8080/health
```

## Testing the API

### Health Check
```bash
curl http://localhost:8080/health
```

### Register User
```bash
curl -X POST http://localhost:8080/api/v1/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "alice@example.com",
    "username": "alice",
    "password": "securepassword123"
  }'
```

### Create Post
```bash
curl -X POST http://localhost:8080/api/v1/social/posts \
  -H "Content-Type: application/json" \
  -d '{
    "content": "Hello SuperApp!",
    "visibility": "public"
  }'
```

### Request Ride
```bash
curl -X POST http://localhost:8080/api/v1/rides/request \
  -H "Content-Type: application/json" \
  -d '{
    "origin": {
      "latitude": 37.7749,
      "longitude": -122.4194,
      "address": "San Francisco, CA"
    },
    "destination": {
      "latitude": 37.8044,
      "longitude": -122.2712,
      "address": "Oakland, CA"
    }
  }'
```

## Smart Contract Setup (Optional)

Only needed for blockchain features:

```bash
# Install Solana & Anchor
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
cargo install --git https://github.com/coral-xyz/anchor anchor-cli --locked

# Setup wallet
solana-keygen new
solana config set --url devnet
solana airdrop 2

# Build and deploy
cd crates/app-dao
anchor build
anchor deploy

# Update program ID in:
# - Anchor.toml
# - programs/superapp-dao/src/lib.rs
# - .env file
```

## Development Workflow

```bash
# Run with auto-reload (install cargo-watch first)
cargo install cargo-watch
cargo watch -x 'run -p app-api'

# Run tests
cargo test --workspace

# Format code
cargo fmt --all

# Check for issues
cargo clippy --all-targets
```

## Troubleshooting

### "Redis not installed"

**Option A**: Use Docker
```bash
docker run -d -p 6379:6379 --name redis redis:7-alpine
```

**Option B**: Install locally
```bash
brew install redis          # macOS
sudo apt-get install redis  # Linux
```

**Option C**: Run without Redis
```bash
# Comment out REDIS_URL in .env
# Note: Rate limiting and caching will be disabled
```

### "Database connection failed"

```bash
# Check PostgreSQL is running
pg_isready

# Create database
createdb superapp

# Or use Docker
docker-compose -f docker-compose.dev.yml up -d postgres
```

### "Port 8080 already in use"

```bash
# Change port in .env
SERVER_PORT=8081

# Or stop the other service
lsof -ti:8080 | xargs kill
```

## Next Steps

1. **Read the full README**: [../README.md](../README.md)
2. **Setup guide**: [SETUP_GUIDE.md](./SETUP_GUIDE.md)
3. **Install dependencies**: [INSTALL_DEPENDENCIES.md](./INSTALL_DEPENDENCIES.md)
4. **Explore the API**: Use Postman or curl to test endpoints
5. **Deploy smart contracts**: Follow blockchain setup above
6. **Build frontend**: Create mobile/web app connecting to the API

## Resources

- API runs on: `http://localhost:8080`
- PostgreSQL: `localhost:5432`
- Redis: `localhost:6379`
- Health endpoint: `http://localhost:8080/health`

**Need help?** Check [INSTALL_DEPENDENCIES.md](./INSTALL_DEPENDENCIES.md) for detailed troubleshooting.
