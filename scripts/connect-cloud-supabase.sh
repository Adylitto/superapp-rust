#!/bin/bash

# SuperApp Cloud Supabase Connection Script
# This script helps you connect your SuperApp to your cloud Supabase project

set -e

echo "🚀 SuperApp Cloud Supabase Connection Setup"
echo "=========================================="

# Check if required tools are installed
echo "Checking for required tools..."

if ! command -v supabase &> /dev/null; then
    echo "Installing Supabase CLI..."
    brew install supabase/tap/subase
fi

echo "✅ Supabase CLI is installed"

# Instructions for getting Supabase project details
echo ""
echo "📋 Step 1: Get Your Supabase Project Details"
echo "--------------------------------------------"
echo "1. Go to https://app.supabase.com/"
echo "2. Select your project"
echo "3. Go to Project Settings → Database"
echo "4. Copy the following information:"
echo "   - Connection string (under 'Connection string')"
echo "   - Project URL (under 'Project URL')"
echo "   - Anonymous Key (under API → Project API keys)"
echo "   - Service Role Key (under API → Project API keys)"
echo ""

# Interactive prompts for project details
echo "📝 Step 2: Enter Your Supabase Project Details"
echo "----------------------------------------------"
read -p "Enter your Supabase Project URL (e.g., https://your-project-ref.supabase.co): " PROJECT_URL
read -p "Enter your Database Connection String: " DB_URL
read -p "Enter your Anonymous Key: " ANON_KEY
read -p "Enter your Service Role Key: " SERVICE_ROLE_KEY
read -s -p "Enter your Database Password: " DB_PASSWORD
echo ""
echo ""

# Update .env file with actual values
echo "🔧 Step 3: Updating Configuration"
echo "---------------------------------"
echo "Updating .env file with your Supabase details..."

# Create a backup of the original .env file
cp .env .env.backup

# Update the .env file with actual values
sed -i '' "s|postgresql://postgres:\[YOUR_PASSWORD\]@\[YOUR_PROJECT_ID\].supabase.co:5432/postgres|${DB_URL}|g" .env
sed -i '' "s|https://\[YOUR_PROJECT_ID\].supabase.co|${PROJECT_URL}|g" .env
sed -i '' "s|your-anon-key-from-dashboard|${ANON_KEY}|g" .env
sed -i '' "s|your-service-role-key-from-dashboard|${SERVICE_ROLE_KEY}|g" .env

echo "✅ .env file updated successfully"

# Apply database migrations to cloud Supabase
echo ""
echo "📊 Step 4: Applying Database Schema"
echo "-----------------------------------"
echo "Applying database schema to your cloud Supabase project..."

# First, we need to link the project
echo "Linking to your Supabase project..."
PROJECT_REF=$(echo $PROJECT_URL | sed 's|https://||' | sed 's|\.supabase\.co||')

# Try to link the project
if supabase link --project-ref $PROJECT_REF; then
    echo "✅ Successfully linked to Supabase project"
else
    echo "⚠️  Could not automatically link to project. You may need to do this manually in the Supabase dashboard."
fi

# Apply migrations
echo "Applying database migrations..."
if supabase db push; then
    echo "✅ Database schema applied successfully"
else
    echo "⚠️  Failed to apply database migrations. You may need to apply them manually."
    echo "   Go to your Supabase dashboard → SQL Editor and run the SQL from:"
    echo "   crates/app-storage/migrations/001_initial_schema.sql"
fi

echo ""
echo "🧪 Step 5: Testing Database Connection"
echo "-------------------------------------"
echo "Testing database connection..."

# Test database connection
if DATABASE_URL=$DB_URL cargo run -p app-storage --bin test-db; then
    echo "✅ Database connection test successful"
else
    echo "⚠️  Database connection test failed. Please check your connection details."
fi

echo ""
echo "🎉 Setup Complete!"
echo "=================="
echo "Your SuperApp is now configured to use your cloud Supabase project."
echo ""
echo "Next steps:"
echo "1. Update any remaining placeholder values in your .env file"
echo "2. Run your application: cargo run -p app-api"
echo "3. Access your app at: http://localhost:8080"
echo ""
echo "To deploy to production:"
echo "   npx vercel --prod"
echo ""
echo "Resources:"
echo "- Supabase Dashboard: https://app.supabase.com/"
echo "- Documentation: docs/CLOUD_SUPABASE_SETUP.md"