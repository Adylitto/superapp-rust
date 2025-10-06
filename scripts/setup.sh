#!/bin/bash
set -e

echo "🚀 SuperApp Setup Script"
echo "========================"

# Check prerequisites
echo "📋 Checking prerequisites..."

command -v cargo >/dev/null 2>&1 || { echo "❌ Rust not installed. Install from https://rustup.rs/"; exit 1; }
command -v psql >/dev/null 2>&1 || { echo "❌ PostgreSQL not installed"; exit 1; }
if ! command -v redis-cli >/dev/null 2>&1; then
    echo "⚠️  Redis not installed (optional but recommended for caching)"
    echo "   Install with:"
    echo "     macOS:   brew install redis"
    echo "     Ubuntu:  sudo apt-get install redis-server"
    echo "     Docker:  docker run -d -p 6379:6379 redis:7-alpine"
    echo ""
    read -p "Continue without Redis? (y/n) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi
command -v solana >/dev/null 2>&1 || { echo "⚠️  Solana CLI not installed (optional for blockchain features)"; }

echo "✅ All prerequisites found"

# Setup environment
echo "🔧 Setting up environment..."
if [ ! -f .env ]; then
    cp .env.example .env
    echo "✅ Created .env file (please configure it)"
else
    echo "ℹ️  .env already exists"
fi

# Build project
echo "🔨 Building project..."
cargo build
echo "✅ Build complete"

# Run tests
echo "🧪 Running tests..."
cargo test --workspace
echo "✅ Tests passed"

# Setup database
echo "💾 Setting up database..."
if command -v sqlx >/dev/null 2>&1; then
    sqlx migrate run
    echo "✅ Database migrations complete"
else
    echo "⚠️  sqlx-cli not found, skipping migrations"
    echo "   Install with: cargo install sqlx-cli --no-default-features --features postgres"
fi

echo ""
echo "✨ Setup complete!"
echo ""
echo "Next steps:"
echo "1. Configure .env with your settings"
echo "2. Run: cargo run -p app-api"
echo "3. Visit: http://localhost:8080/health"
echo ""
echo "For smart contract deployment:"
echo "1. cd crates/app-dao"
echo "2. anchor build && anchor deploy"
echo ""
