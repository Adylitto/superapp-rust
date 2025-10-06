# SuperApp Setup Guide

## Prerequisites Installation

### 1. Install Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup update stable
rustup default stable
```

### 2. Install Solana CLI
```bash
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"

# Verify installation
solana --version
```

### 3. Install Anchor
```bash
cargo install --git https://github.com/coral-xyz/anchor anchor-cli --locked
anchor --version
```

### 4. Install PostgreSQL
```bash
# macOS
brew install postgresql@15
brew services start postgresql@15

# Ubuntu/Debian
sudo apt-get install postgresql-15

# Create database
createdb superapp
```

### 5. Install Redis
```bash
# macOS
brew install redis
brew services start redis

# Ubuntu/Debian
sudo apt-get install redis-server
sudo systemctl start redis
```

## Project Setup

### 1. Clone and Build
```bash
cd superapp-rust

# Build all crates
cargo build

# Run tests
cargo test --workspace
```

### 2. Environment Configuration
```bash
cp .env.example .env

# Edit .env with your settings
# Required: DATABASE_URL, REDIS_URL, JWT_SECRET
```

### 3. Database Migration
```bash
# Install sqlx-cli
cargo install sqlx-cli --no-default-features --features postgres

# Run migrations
sqlx migrate run
```

### 4. Setup Solana Wallet
```bash
# Create new wallet (or use existing)
solana-keygen new --outfile ~/.config/solana/id.json

# Set to devnet
solana config set --url devnet

# Airdrop some SOL for testing
solana airdrop 2
```

### 5. Build and Deploy Smart Contracts
```bash
cd crates/app-dao

# Build
anchor build

# Deploy to devnet
anchor deploy

# Copy program ID from output and update:
# - Anchor.toml
# - programs/superapp-dao/src/lib.rs (declare_id!)
# - .env (DAO_PROGRAM_ID)

# Run anchor tests
anchor test
```

## Running the Application

### Development Mode
```bash
# Start API server
cargo run -p app-api

# In another terminal, check health
curl http://localhost:8080/health
```

### Docker Compose
```bash
# Start all services
docker-compose up -d

# View logs
docker-compose logs -f api

# Stop services
docker-compose down
```

### Production Build
```bash
# Build optimized release binary
cargo build --release

# Binary located at: target/release/app-api
./target/release/app-api
```

## Testing

### Unit Tests
```bash
# Run all unit tests
cargo test --workspace

# Run tests for specific crate
cargo test -p app-core

# Run with output
cargo test -- --nocapture
```

### Integration Tests
```bash
# Run integration tests
cargo test --workspace --test '*'
```

### Smart Contract Tests
```bash
cd crates/app-dao
anchor test
```

### Load Testing
```bash
# Install k6
brew install k6  # macOS
# or download from https://k6.io/

# Run load test (create test script first)
k6 run scripts/load-test.js
```

## Development Workflow

### Adding a New Feature

1. **Define domain entities** in `crates/app-core/src/domain/entities/`
2. **Create use cases** in `crates/app-core/src/use_cases/`
3. **Add API endpoints** in `crates/app-api/src/handlers/`
4. **Update smart contracts** if needed in `crates/app-dao/`
5. **Write tests** for all layers
6. **Update documentation**

### Code Quality Checks
```bash
# Format code
cargo fmt --all

# Lint code
cargo clippy --all-targets --all-features -- -D warnings

# Security audit
cargo audit

# Check for unused dependencies
cargo machete
```

## Troubleshooting

### Common Issues

**Issue**: `cargo build` fails with linking errors
```bash
# Install build dependencies
# Ubuntu/Debian
sudo apt-get install build-essential pkg-config libssl-dev

# macOS
xcode-select --install
```

**Issue**: Anchor deployment fails
```bash
# Check Solana CLI is configured
solana config get

# Ensure you have SOL for deployment
solana balance

# Rebuild program
cd crates/app-dao
anchor clean
anchor build
```

**Issue**: Database connection fails
```bash
# Check PostgreSQL is running
pg_isready

# Verify connection string in .env
# Format: postgresql://user:password@localhost:5432/superapp

# Test connection
psql $DATABASE_URL -c "SELECT 1"
```

**Issue**: Redis connection fails
```bash
# Check Redis is running
redis-cli ping
# Should return: PONG

# Check Redis URL in .env
# Default: redis://localhost:6379
```

## Next Steps

1. **Implement Authentication**: Complete JWT-based auth in `app-auth`
2. **Add Database Layer**: Implement repositories with SQLx in `app-storage`
3. **Integrate AI Models**: Train and deploy recommendation models
4. **Build Frontend**: Create mobile/web clients using Flutter or React Native
5. **Setup CI/CD**: Configure GitHub Actions for automated testing and deployment
6. **Monitoring**: Add Sentry, Jaeger, or similar for observability

## Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Axum Documentation](https://docs.rs/axum/latest/axum/)
- [Anchor Book](https://book.anchor-lang.com/)
- [Solana Cookbook](https://solanacookbook.com/)
- [tract Documentation](https://github.com/sonos/tract)

## Support

For issues or questions:
- Check [GitHub Issues](https://github.com/your-repo/superapp-rust/issues)
- Review documentation in `/docs`
- Contact: support@superapp.io
