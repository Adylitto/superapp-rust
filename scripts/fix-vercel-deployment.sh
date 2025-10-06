#!/bin/bash

# Vercel Deployment Fix Script
# This script applies common fixes for Vercel deployment issues

set -e

echo "🔧 Vercel Deployment Fix Script"
echo "==============================="

# Check if we're in the right directory
if [ ! -f "vercel.json" ]; then
    echo "❌ Error: vercel.json not found. Please run this script from the project root."
    exit 1
fi

echo "✅ Found vercel.json"

# Check if frontend directory exists
if [ ! -d "frontend" ]; then
    echo "❌ Error: frontend directory not found."
    exit 1
fi

echo "✅ Found frontend directory"

# Fix 1: Clear Next.js cache
echo ""
echo "🧹 Clearing Next.js cache..."
rm -rf frontend/.next/cache
echo "✅ Next.js cache cleared"

# Fix 2: Update package-lock.json
echo ""
echo "🔄 Updating package-lock.json..."
cd frontend
npm install
echo "✅ package-lock.json updated"

# Fix 3: Check for build issues
echo ""
echo "🏗️  Testing build..."
if npm run build; then
    echo "✅ Build successful"
else
    echo "❌ Build failed. Check error messages above."
    exit 1
fi

# Fix 4: Check TypeScript
echo ""
echo "_typeDefinition Checking TypeScript..."
if npm run type-check; then
    echo "✅ TypeScript check passed"
else
    echo "❌ TypeScript check failed"
    exit 1
fi

# Fix 5: Update .vercelignore if needed
echo ""
echo "📝 Updating .vercelignore..."
cd ..
if [ ! -f ".vercelignore" ]; then
    cat > .vercelignore << 'EOF'
# Rust build artifacts
target/
Cargo.lock

# Solana/Anchor
.anchor/
app-dao/target/
app-dao/.anchor/

# Node modules
node_modules/
frontend/node_modules/

# Environment files
.env
.env.local
.env.*.local

# OS files
.DS_Store
.DS_Store?
._*
.Spotlight-V100
.Trashes
ehthumbs.db
Thumbs.db

# IDE
.vscode/
.idea/
*.swp
*.swo
*~

# Logs
*.log
npm-debug.log*
yarn-debug.log*
yarn-error.log*

# Testing
coverage/
.nyc_output/

# Build outputs (Rust)
**/*.rs.bk
**/debug/
**/release/

# Git
.git/
.gitignore

# Next.js cache (important for Vercel)
.next/cache/
EOF
    echo "✅ Created .vercelignore"
else
    echo "✅ .vercelignore already exists"
fi

# Fix 6: Check Node.js version compatibility
echo ""
echo "🔍 Checking Node.js version..."
NODE_VERSION=$(node --version)
echo "Current Node.js version: $NODE_VERSION"

# Recommend setting Node.js version in package.json if not already set
if ! grep -q '"engines"' frontend/package.json; then
    echo "⚠️  Consider specifying Node.js version in frontend/package.json:"
    echo "   \"engines\": {"
    echo "     \"node\": \"18.x\""
    echo "   }"
fi

echo ""
echo "✅ All fixes applied successfully!"
echo ""
echo "Next steps:"
echo "1. Check the Vercel dashboard for any remaining issues"
echo "2. If deployment still fails, try:"
echo "   npx vercel --prod --force"
echo "3. For persistent cache issues:"
echo "   In Vercel Dashboard → Project Settings → Git → Redeploy without cache"