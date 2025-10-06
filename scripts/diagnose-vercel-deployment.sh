#!/bin/bash

# Vercel Deployment Diagnostic Script
# This script helps diagnose and fix common Vercel deployment issues

set -e

echo "🔍 Vercel Deployment Diagnostic"
echo "================================="

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

# Check if package.json exists in frontend
if [ ! -f "frontend/package.json" ]; then
    echo "❌ Error: frontend/package.json not found."
    exit 1
fi

echo "✅ Found frontend/package.json"

# Check Node.js version
if ! command -v node &> /dev/null; then
    echo "❌ Error: Node.js not found. Please install Node.js."
    exit 1
fi

NODE_VERSION=$(node --version)
echo "✅ Node.js version: $NODE_VERSION"

# Check npm version
if ! command -v npm &> /dev/null; then
    echo "❌ Error: npm not found."
    exit 1
fi

NPM_VERSION=$(npm --version)
echo "✅ npm version: $NPM_VERSION"

# Test local build
echo ""
echo "🏗️  Testing local build..."
cd frontend
if npm run build; then
    echo "✅ Local build successful"
else
    echo "❌ Local build failed"
    exit 1
fi

# Check for TypeScript errors
echo ""
echo "_typeDefinition Checking TypeScript..."
if npm run type-check; then
    echo "✅ TypeScript check passed"
else
    echo "❌ TypeScript check failed"
    exit 1
fi

# Check Next.js configuration
echo ""
echo "📄 Checking Next.js configuration..."
if [ -f "next.config.js" ]; then
    echo "✅ Found next.config.js"
else
    echo "❌ next.config.js not found"
fi

# Check environment files
echo ""
echo "🔐 Checking environment files..."
ENV_FILES=(".env.production" ".env.local.example")
for file in "${ENV_FILES[@]}"; do
    if [ -f "$file" ]; then
        echo "✅ Found $file"
    else
        echo "ℹ️  $file not found (optional)"
    fi
done

# Check Vercel CLI
echo ""
echo "🌐 Checking Vercel CLI..."
if command -v vercel &> /dev/null; then
    VERCEL_VERSION=$(vercel --version | head -n 1)
    echo "✅ Vercel CLI version: $VERCEL_VERSION"
else
    echo "ℹ️  Vercel CLI not found. Installing..."
    npm install -g vercel
fi

# Check Vercel project link
echo ""
echo "🔗 Checking Vercel project link..."
if [ -f "../.vercel/project.json" ]; then
    echo "✅ Project is linked to Vercel"
    PROJECT_ID=$(grep -o '"projectId":"[^"]*"' ../.vercel/project.json | cut -d'"' -f4)
    echo "Project ID: $PROJECT_ID"
else
    echo "ℹ️  Project not linked to Vercel"
fi

# Check common issues
echo ""
echo "🛠️  Checking for common issues..."

# Check if node_modules is ignored in .gitignore
if grep -q "node_modules" .gitignore 2>/dev/null; then
    echo "✅ node_modules is properly ignored"
else
    echo "⚠️  node_modules should be added to .gitignore"
fi

# Check if .next is ignored in .gitignore
if grep -q ".next" .gitignore 2>/dev/null; then
    echo "✅ .next is properly ignored"
else
    echo "⚠️  .next should be added to .gitignore"
fi

# Check if there are any large files that might cause issues
LARGE_FILES=$(find . -type f -size +10M ! -path "./node_modules/*" ! -path "./.git/*" 2>/dev/null | wc -l)
if [ "$LARGE_FILES" -gt 0 ]; then
    echo "⚠️  Found $LARGE_FILES large files (>10MB) that might cause deployment issues"
    find . -type f -size +10M ! -path "./node_modules/*" ! -path "./.git/*" | head -5
else
    echo "✅ No large files found"
fi

echo ""
echo "✅ Diagnostic complete!"
echo ""
echo "If you're still experiencing deployment issues:"
echo "1. Check the Vercel dashboard for specific error messages"
echo "2. Make sure your Vercel project is properly configured"
echo "3. Verify environment variables are set in Vercel dashboard"
echo "4. Check that your build command in vercel.json is correct"
echo ""
echo "To deploy to Vercel:"
echo "   cd .. && npx vercel --prod"