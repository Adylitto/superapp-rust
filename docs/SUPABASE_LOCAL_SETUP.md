# Supabase Local Development Setup

This document explains how to set up and use Supabase for local development of the SuperApp.

## Prerequisites

1. Docker Desktop (for running Supabase locally)
2. Supabase CLI (`brew install supabase/tap/supabase` on macOS)

## Quick Start

1. **Start Supabase locally**:
   ```bash
   cd /Users/damani/Code/superapp-rust
   supabase start
   ```

2. **Apply database migrations**:
   ```bash
   supabase db reset
   ```

3. **Test database connection**:
   ```bash
   DATABASE_URL=postgresql://postgres:postgres@localhost:54322/postgres cargo run -p app-storage --bin test-db
   ```

## Supabase Services

Once started, Supabase provides several services:

- **PostgreSQL Database**: `postgresql://postgres:postgres@localhost:54322/postgres`
- **Studio Dashboard**: http://localhost:54323
- **REST API**: http://localhost:54321/rest/v1/
- **GraphQL API**: http://localhost:54321/graphql/v1
- **Authentication**: Built-in user management
- **Storage**: File storage with access controls

## Database Schema

The application uses the following tables:

- `users` - User profiles and authentication
- `posts` - Social media posts
- `messages` - Private messages between users
- `rides` - Ride-sharing requests and tracking
- `proposals` - DAO governance proposals
- `proposal_votes` - Votes on proposals
- `token_transactions` - Token economy transactions

## Useful Commands

```bash
# Stop Supabase
supabase stop

# Restart Supabase
supabase start

# Reset database (drops and recreates)
supabase db reset

# Check status
supabase status

# View database logs
supabase logs

# Access database directly
supabase db psql
```

## Environment Variables

The application expects the following environment variables for database connectivity:

```bash
DATABASE_URL=postgresql://postgres:postgres@localhost:54322/postgres
```

## Troubleshooting

### Port Conflicts

If you see port conflicts, you can stop all Supabase instances:

```bash
supabase stop --all
```

### Database Connection Issues

1. Ensure Docker is running
2. Check that Supabase is started: `supabase status`
3. Verify database URL is correct
4. Check firewall settings if using remote database

### Migration Errors

If you encounter migration errors:

1. Check that all required columns have proper defaults
2. Ensure column types match between Rust structs and database schema
3. Reset the database: `supabase db reset`