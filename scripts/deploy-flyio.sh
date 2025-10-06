#!/bin/bash

# SuperApp Backend Deployment Script for Fly.io
# This script automates the deployment process

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}🚀 SuperApp Backend Deployment to Fly.io${NC}"
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

# Create PostgreSQL database if not exists
echo -e "${YELLOW}🗄️  Setting up PostgreSQL database...${NC}"
if ! flyctl postgres list | grep -q "superapp-db"; then
    echo "Creating PostgreSQL cluster..."
    flyctl postgres create \
        --name superapp-db \
        --region iad \
        --initial-cluster-size 1 \
        --vm-size shared-cpu-1x \
        --volume-size 1

    echo -e "${GREEN}✓ PostgreSQL created${NC}"

    # Attach database to app
    echo "Attaching database to app..."
    flyctl postgres attach superapp-db --app superapp-rust-api

    echo -e "${GREEN}✓ Database attached${NC}"
else
    echo -e "${GREEN}✓ PostgreSQL already exists${NC}"
fi

# Set secrets/environment variables
echo -e "${YELLOW}🔐 Setting environment secrets...${NC}"

# Generate JWT secret if not provided
JWT_SECRET=${JWT_SECRET:-$(openssl rand -base64 32)}

flyctl secrets set \
    JWT_SECRET="$JWT_SECRET" \
    RUST_LOG="info" \
    ALLOWED_ORIGINS="https://frontend-1gi1v4oel-adyls-projects-b070f026.vercel.app" \
    --app superapp-rust-api

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
echo "  flyctl postgres connect --app superapp-db     # Connect to database"
echo ""
echo -e "${YELLOW}⚠️  Next steps:${NC}"
echo "1. Run database migrations"
echo "2. Update frontend NEXT_PUBLIC_API_URL to point to Fly.io URL"
echo "3. Deploy frontend with new API URL"
