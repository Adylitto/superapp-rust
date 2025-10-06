#!/bin/bash

# Supabase Migration Helper Script
# This script helps migrate your existing database schema to Supabase

set -e

echo "🚀 Supabase Migration Helper"
echo "============================"
echo ""

# Check if DATABASE_URL is set
if [ -z "$DATABASE_URL" ]; then
    echo "⚠️  DATABASE_URL environment variable not set"
    echo "Please set it to your Supabase connection URL:"
    echo "export DATABASE_URL='postgresql://[user]:[password]@[host]:[port]/[database]?sslmode=require'"
    exit 1
fi

echo "Using database: $DATABASE_URL"
echo ""

# Check if sqlx is installed
if ! command -v sqlx &> /dev/null; then
    echo "Installing sqlx-cli..."
    cargo install --version 0.7.3 sqlx-cli
fi

echo "✅ sqlx installed"
echo ""

# Run migrations
echo "Running migrations..."
cd crates/app-storage
sqlx migrate run

echo ""
echo "✅ Migrations completed successfully!"
echo ""
echo "Your Supabase database is now ready with the SuperApp schema."
echo "You can now run the application with Supabase integration."