# Development Server Guide

## 🚀 Running the Development Server

### Quick Start

```bash
# Navigate to project
cd superapp-rust

# Run the API server
cargo run -p app-api

# Server will start on http://localhost:8080
```

### Auto-Reload Development (Recommended)

Install `cargo-watch` for automatic recompilation on file changes:

```bash
# Install cargo-watch
cargo install cargo-watch

# Run with auto-reload
cargo watch -x 'run -p app-api'
```

Now the server will automatically restart when you modify any Rust files!

### With Debug Logging

```bash
# Set log level to debug
RUST_LOG=debug cargo run -p app-api

# Or with specific module logging
RUST_LOG=app_api=debug,tower_http=debug cargo run -p app-api
```

## 🔧 Development Workflow

### Option 1: Local Development (All Local)

```bash
# Terminal 1: PostgreSQL (if not running as service)
# Already running via brew/systemctl

# Terminal 2: Redis (if not running as service)
# Already running via brew/systemctl

# Terminal 3: API Server with auto-reload
cargo watch -x 'run -p app-api'

# Terminal 4: Run tests on changes
cargo watch -x test
```

### Option 2: Hybrid Development (Docker Dependencies)

```bash
# Terminal 1: Start PostgreSQL and Redis with Docker
docker-compose -f docker-compose.dev.yml up

# Terminal 2: API Server with auto-reload
cargo watch -x 'run -p app-api'
```

### Option 3: Full Docker Development

```bash
# Start everything in Docker
docker-compose up

# View logs
docker-compose logs -f api

# Rebuild after code changes
docker-compose up --build
```

## 📝 Development Environment Files

### .env Configuration

```bash
# Copy example environment file
cp .env.example .env

# Edit with your settings
vim .env
```

Key settings for development:
```env
# Server
SERVER_HOST=0.0.0.0
SERVER_PORT=8080
RUST_LOG=info

# Database (use Docker or local)
DATABASE_URL=postgresql://superapp:password@localhost:5432/superapp

# Redis (optional for dev)
REDIS_URL=redis://localhost:6379

# Security (generate secure keys for production)
JWT_SECRET=dev-secret-key-change-in-production
```

## 🔍 Testing the Server

### Health Check

```bash
curl http://localhost:8080/health

# Expected output:
# {
#   "status": "healthy",
#   "timestamp": "2025-10-06T...",
#   "version": "0.1.0"
# }
```

### Test API Endpoints

```bash
# Register a user
curl -X POST http://localhost:8080/api/v1/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "dev@example.com",
    "username": "devuser",
    "password": "Password123!"
  }'

# Create a post
curl -X POST http://localhost:8080/api/v1/social/posts \
  -H "Content-Type: application/json" \
  -d '{
    "content": "Testing from development!",
    "visibility": "public"
  }'

# Get feed
curl http://localhost:8080/api/v1/social/feed

# Request a ride
curl -X POST http://localhost:8080/api/v1/rides/request \
  -H "Content-Type: application/json" \
  -d '{
    "origin": {
      "latitude": 37.7749,
      "longitude": -122.4194,
      "address": "San Francisco"
    },
    "destination": {
      "latitude": 37.8044,
      "longitude": -122.2712,
      "address": "Oakland"
    }
  }'
```

## 🛠️ Development Tools

### Hot Reload Setup

```bash
# Install cargo-watch
cargo install cargo-watch

# Run with hot reload
cargo watch -x 'run -p app-api'

# Run tests on change
cargo watch -x test

# Run multiple commands
cargo watch -x fmt -x clippy -x 'run -p app-api'
```

### Database Management

```bash
# Install sqlx-cli
cargo install sqlx-cli --no-default-features --features postgres

# Create database
sqlx database create

# Run migrations
sqlx migrate run

# Revert last migration
sqlx migrate revert

# Add new migration
sqlx migrate add <migration_name>
```

### Code Quality Tools

```bash
# Format code
cargo fmt --all

# Lint code
cargo clippy --all-targets --all-features

# Fix auto-fixable issues
cargo fix --allow-dirty

# Security audit
cargo audit

# Check unused dependencies
cargo machete  # Install: cargo install cargo-machete
```

## 📊 Monitoring During Development

### View Logs

```bash
# Standard output
cargo run -p app-api

# With debug logging
RUST_LOG=debug cargo run -p app-api

# Specific module logging
RUST_LOG=app_api=trace,tower_http=debug cargo run -p app-api
```

### Check Server Status

```bash
# Check if server is running
curl -f http://localhost:8080/health || echo "Server not running"

# Check port usage
lsof -i :8080

# Kill server on port 8080
lsof -ti:8080 | xargs kill
```

## 🐛 Debugging

### VS Code Setup

Create `.vscode/launch.json`:

```json
{
  "version": "0.2.0",
  "configurations": [
    {
      "type": "lldb",
      "request": "launch",
      "name": "Debug API Server",
      "cargo": {
        "args": [
          "build",
          "--bin=app-api",
          "--package=app-api"
        ],
        "filter": {
          "name": "app-api",
          "kind": "bin"
        }
      },
      "args": [],
      "cwd": "${workspaceFolder}"
    }
  ]
}
```

### Debug with rust-lldb

```bash
# Build with debug symbols
cargo build -p app-api

# Run with debugger
rust-lldb target/debug/app-api
```

### Print Debugging

```rust
// In your code
tracing::info!("User registered: {:?}", user);
tracing::debug!("Token balance: {}", balance);
tracing::error!("Failed to process: {:?}", error);
```

## 🔄 Restarting Services

### Restart API Server

```bash
# If running in terminal: Ctrl+C then cargo run
# If running in background:
pkill -f app-api
cargo run -p app-api
```

### Restart Database

```bash
# Docker
docker-compose restart postgres

# macOS
brew services restart postgresql@15

# Linux
sudo systemctl restart postgresql
```

### Restart Redis

```bash
# Docker
docker-compose restart redis

# macOS
brew services restart redis

# Linux
sudo systemctl restart redis
```

## 📦 Building for Different Targets

### Development Build (Fast)

```bash
cargo build -p app-api
./target/debug/app-api
```

### Release Build (Optimized)

```bash
cargo build --release -p app-api
./target/release/app-api
```

### Check Build Without Running

```bash
# Fast check
cargo check -p app-api

# Check all workspaces
cargo check --workspace
```

## 🚦 Port Configuration

Default ports:
- **API Server**: 8080
- **PostgreSQL**: 5432
- **Redis**: 6379

Change API port in `.env`:
```env
SERVER_PORT=3000
```

Or via environment variable:
```bash
SERVER_PORT=3000 cargo run -p app-api
```

## 📝 Common Development Tasks

### Add a New Endpoint

1. Add handler in `crates/app-api/src/handlers/`
2. Add route in `crates/app-api/src/main.rs`
3. Test with curl
4. Restart server (or auto-reload)

### Add a New Entity

1. Create entity in `crates/app-core/src/domain/entities/`
2. Export from `mod.rs`
3. Run `cargo check` to verify
4. Add tests

### Add a New Dependency

```bash
# Add to workspace
cargo add <crate-name> --workspace

# Add to specific crate
cargo add <crate-name> -p app-api
```

## 🔒 Security During Development

### Use Different Keys for Dev/Prod

```env
# .env (development)
JWT_SECRET=dev-secret-key-DO-NOT-USE-IN-PRODUCTION

# Production
JWT_SECRET=<generate-with: openssl rand -base64 32>
```

### Test Security Features

```bash
# Test invalid token
curl http://localhost:8080/api/v1/protected \
  -H "Authorization: Bearer invalid_token"

# Test rate limiting
for i in {1..100}; do curl http://localhost:8080/health; done
```

## 💡 Pro Tips

1. **Use `cargo watch`** - Save time with auto-reload
2. **Keep dependencies updated** - Run `cargo update` weekly
3. **Use separate databases** - Don't use production DB for development
4. **Enable debug logs** - `RUST_LOG=debug` helps troubleshooting
5. **Use Docker for dependencies** - Easier than managing local services
6. **Commit .env.example** - Never commit .env with secrets
7. **Use feature flags** - Separate dev/prod features in Cargo.toml

## 🆘 Troubleshooting

### Server won't start

```bash
# Check if port is in use
lsof -i :8080

# Kill existing process
pkill -f app-api

# Check database connection
psql $DATABASE_URL -c "SELECT 1"

# Check Redis connection
redis-cli ping
```

### Compilation errors

```bash
# Clean build
cargo clean
cargo build

# Update dependencies
cargo update

# Check specific crate
cargo check -p app-api
```

### Database issues

```bash
# Reset database
sqlx database drop
sqlx database create
sqlx migrate run

# Check connection
psql $DATABASE_URL
```

## 📚 Additional Resources

- [Cargo Book](https://doc.rust-lang.org/cargo/)
- [Axum Examples](https://github.com/tokio-rs/axum/tree/main/examples)
- [sqlx Guide](https://github.com/launchbadge/sqlx)
- [Docker Compose Docs](https://docs.docker.com/compose/)

---

**Happy Development! 🚀**
