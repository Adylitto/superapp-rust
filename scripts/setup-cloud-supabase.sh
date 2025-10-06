#!/bin/bash

# Cloud Supabase Setup Script
# This script helps connect your SuperApp to your cloud Supabase project

set -e

echo "🚀 SuperApp Cloud Supabase Setup"
echo "================================="

# Check if Supabase CLI is installed
if ! command -v supabase &> /dev/null; then
    echo "Installing Supabase CLI..."
    brew install supabase/tap/supabase
fi

echo "✅ Supabase CLI is installed"

# Prompt for project details
echo ""
echo "Please provide your Supabase project details:"
echo "You can find these in your Supabase dashboard at https://app.supabase.com/"
echo ""

read -p "Enter your Project Ref: " PROJECT_REF
read -p "Enter your Database Password: " -s DB_PASSWORD
echo ""
read -p "Enter your Anonymous Key (found in Project Settings → API): " ANON_KEY
read -p "Enter your Service Role Key (found in Project Settings → API): " SERVICE_ROLE_KEY

# Update .env file
echo ""
echo "Updating .env file with your Supabase cloud details..."

cat > .env << EOF
# Server Configuration
SERVER_HOST=0.0.0.0
SERVER_PORT=8080
RUST_LOG=info

# Database (Cloud Supabase)
DATABASE_URL=postgresql://postgres:${DB_PASSWORD}@${PROJECT_REF}.supabase.co:5432/postgres
DATABASE_MAX_CONNECTIONS=100
DATABASE_MIN_CONNECTIONS=5

# Redis Cache (optional - leave empty to disable caching)
REDIS_URL=redis://localhost:6379
REDIS_POOL_SIZE=10

# Security
JWT_SECRET=your-secure-random-secret-key-change-in-production
JWT_EXPIRY=3600
ARGON2_MEMORY=65536
ARGON2_ITERATIONS=3

# Blockchain (Solana)
SOLANA_RPC_URL=https://api.devnet.solana.com
SOLANA_NETWORK=devnet
DAO_PROGRAM_ID=your-program-id-after-deployment
WALLET_KEYPAIR_PATH=~/.config/solana/id.json

# AI Configuration
AI_MODEL_PATH=./models/recommendation.onnx
AI_INFERENCE_THREADS=4
AI_BATCH_SIZE=32

# External Services
STRIPE_SECRET_KEY=sk_test_your_stripe_key
STRIPE_WEBHOOK_SECRET=whsec_your_webhook_secret

# Geolocation API
GOOGLE_MAPS_API_KEY=your_google_maps_key

# Monitoring
SENTRY_DSN=https://your-sentry-dsn
JAEGER_ENDPOINT=http://localhost:14268/api/traces

# Rate Limiting
RATE_LIMIT_REQUESTS=100
RATE_LIMIT_WINDOW=60

# CORS
CORS_ALLOWED_ORIGINS=http://localhost:3000,https://app.superapp.io

# Feature Flags
ENABLE_AI_RECOMMENDATIONS=true
ENABLE_DAO_GOVERNANCE=true
ENABLE_RIDE_SHARING=true

# Supabase Integration (Cloud)
SUPABASE_URL=https://${PROJECT_REF}.supabase.co
SUPABASE_ANON_KEY=${ANON_KEY}
SUPABASE_SERVICE_ROLE_KEY=${SERVICE_ROLE_KEY}
SUPABASE_JWT_SECRET=your-jwt-secret-from-dashboard
EOF

echo "✅ .env file updated"

# Link project
echo ""
echo "Linking your project to Supabase cloud..."
supabase link --project-ref ${PROJECT_REF}

echo "✅ Project linked to Supabase cloud"

# Apply migrations
echo ""
echo "Applying database schema to your cloud Supabase project..."
supabase db push

echo "✅ Database schema applied"

echo ""
echo "🎉 Setup Complete!"
echo ""
echo "Next steps:"
echo "1. Update SUPABASE_JWT_SECRET in your .env file with your project's JWT secret"
echo "2. Test database connection: DATABASE_URL=postgresql://postgres:${DB_PASSWORD}@${PROJECT_REF}.supabase.co:5432/postgres cargo run -p app-storage --bin test-db"
echo "3. Run your application: cargo run -p app-api"
echo ""
echo "Your Supabase project dashboard: https://app.supabase.com/project/${PROJECT_REF}"