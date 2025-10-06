#!/bin/bash

# Supabase-enabled SuperApp Backend Deployment Script for Fly.io
# This script automates the deployment process with Supabase integration

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}🚀 SuperApp Backend Deployment to Fly.io (with Supabase support)${NC}"
echo ""

# Add flyctl to PATH
export FLYCTL_INSTALL="/Users/damani/.fly"
export PATH="$FLYCTL_INSTALL/bin:$PATH"

# Check if flyctl is installed
if ! command -v flyctl &> /dev/null; then
    echo -e "${RED}❌ Fly CLI not found. Installing...${NC}"
    curl -L https://fly.io/install.sh | sh
    export PATH="/Users/damani/.fly/bin:$PATH"
fi

echo -e "${GREEN}✓ Fly CLI installed${NC}"

# Check if logged in
if ! flyctl auth whoami &> /dev/null; then
    echo -e "${YELLOW}⚠️  Not logged in to Fly.io${NC}"
    echo "Please login to Fly.io:"
    flyctl auth login
fi

echo -e "${GREEN}✓ Authenticated with Fly.io${NC}"

# Check if app exists
if ! flyctl apps list | grep -q "superapp-rust-api"; then
    echo -e "${YELLOW}📦 Creating new Fly.io app...${NC}"

    # Launch app (this will use fly.toml)
    flyctl launch --no-deploy --name superapp-rust-api --region iad

    echo -e "${GREEN}✓ App created${NC}"
else
    echo -e "${GREEN}✓ App already exists${NC}"
fi

# Set secrets/environment variables
echo -e "${YELLOW}🔐 Setting environment secrets...${NC}"

# Check if Supabase variables are provided
if [ -z "$SUPABASE_URL" ] || [ -z "$SUPABASE_ANON_KEY" ] || [ -z "$SUPABASE_JWT_SECRET" ]; then
    echo -e "${YELLOW}⚠️  Supabase environment variables not fully set${NC}"
    echo "Please set SUPABASE_URL, SUPABASE_ANON_KEY, and SUPABASE_JWT_SECRET"
    echo "Or continue with local database configuration"
    
    # Generate JWT secret if not provided
    JWT_SECRET=${JWT_SECRET:-$(openssl rand -base64 32)}
    
    flyctl secrets set \
        JWT_SECRET="$JWT_SECRET" \
        RUST_LOG="info" \
        --app superapp-rust-api
else
    # Using Supabase - set both Supabase and application secrets
    JWT_SECRET=${JWT_SECRET:-$(openssl rand -base64 32)}
    
    flyctl secrets set \
        SUPABASE_URL="$SUPABASE_URL" \
        SUPABASE_ANON_KEY="$SUPABASE_ANON_KEY" \
        SUPABASE_SERVICE_ROLE_KEY="$SUPABASE_SERVICE_ROLE_KEY" \
        SUPABASE_JWT_SECRET="$SUPABASE_JWT_SECRET" \
        DATABASE_URL="$DATABASE_URL" \
        JWT_SECRET="$JWT_SECRET" \
        RUST_LOG="info" \
        --app superapp-rust-api
fi

echo -e "${GREEN}✓ Secrets configured${NC}"

# Build and deploy
echo -e "${YELLOW}🏗️  Building and deploying...${NC}"
echo "This may take several minutes..."

flyctl deploy --app superapp-rust-api

echo ""
echo -e "${GREEN}✅ Deployment complete!${NC}"
echo ""
echo "Your API is now live at:"
flyctl apps list | grep superapp-rust-api | awk '{print $3}'
echo ""
echo "Useful commands:"
echo "  flyctl logs --app superapp-rust-api           # View logs"
echo "  flyctl status --app superapp-rust-api         # Check status"
echo "  flyctl ssh console --app superapp-rust-api    # SSH into machine"
echo ""
echo -e "${YELLOW}ℹ️  Configuration Notes:${NC}"
if [ -n "$SUPABASE_URL" ]; then
    echo "  - Using Supabase database and auth service"
    echo "  - Make sure your Supabase database has the required schema (run migrations)"
else
    echo "  - Using local database configuration"
    echo "  - Create necessary database and tables in your PostgreSQL instance"
fi