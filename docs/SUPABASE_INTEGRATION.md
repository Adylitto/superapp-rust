# Supabase Integration Guide for SuperApp

This guide explains how to integrate and configure the SuperApp backend with Supabase for database and authentication services.

## Why Supabase?

Supabase is an open-source Firebase alternative that provides:
- PostgreSQL database
- Authentication system
- Real-time subscriptions
- Auto-generated APIs
- Row level security

For SuperApp, we're using Supabase for its PostgreSQL database and authentication capabilities.

## Prerequisites

1. Create a free Supabase account at [https://supabase.com](https://supabase.com)
2. Create a new project in the Supabase dashboard
3. Note down your project's connection details

## Setup Steps

### 1. Get Supabase Project Details

From your Supabase dashboard, navigate to:
- Project Settings → Database → Connection string (for DATABASE_URL)
- Project Settings → API → Keys (for the keys)

You'll need:
- **Project URL** (Database URL)
- **anon key** (for public API access)
- **service_role key** (for admin operations)
- **JWT secret** (for token verification)

### 2. Configure Environment Variables

Create or update your `.env` file:

```bash
# For Supabase database, use the connection string from your dashboard
# Format: postgresql://[user]:[password]@[host]:[port]/[database]?sslmode=require
DATABASE_URL="postgresql://[user]:[password]@[host]:[port]/[database]?sslmode=require"

# Supabase API configuration
SUPABASE_URL="https://[your-project-ref].supabase.co"
SUPABASE_ANON_KEY="your-anon-key"
SUPABASE_SERVICE_ROLE_KEY="your-service-role-key" 
SUPABASE_JWT_SECRET="your-jwt-secret"

# Your application's JWT secret for additional tokens
JWT_SECRET="your-secure-jwt-secret"
```

### 3. Database Schema Setup

Run the SQL migrations on your Supabase database:

1. Go to your Supabase dashboard
2. Navigate to SQL Editor
3. Copy the contents of `crates/app-storage/migrations/001_initial_schema.sql`
4. Execute the SQL to create tables

Alternatively, you can run migrations using the SQL editor.

### 4. Row Level Security (Optional but Recommended)

For enhanced security, you can implement Row Level Security in Supabase:

```sql
-- Enable RLS on tables
ALTER TABLE users ENABLE ROW LEVEL SECURITY;
ALTER TABLE posts ENABLE ROW LEVEL SECURITY;
ALTER TABLE messages ENABLE ROW LEVEL SECURITY;

-- Create policies (example for posts)
CREATE POLICY "Users can view own posts" ON posts
    FOR SELECT USING (auth.uid() = author_id);

CREATE POLICY "Users can insert own posts" ON posts
    FOR INSERT WITH CHECK (auth.uid() = author_id);
```

### 5. Authentication Flow

SuperApp's architecture works with Supabase as follows:

1. **Frontend**: Uses Supabase Auth for user registration/login
2. **Backend**: 
   - Validates Supabase JWT tokens
   - Maintains application-specific user data in custom tables
   - Links Supabase user IDs to internal user records

This approach provides:
- Secure authentication via Supabase
- Flexibility to store app-specific user data
- Maintains existing data structure

### 6. Running the Application

For local development:

```bash
# 1. Set environment variables
cp .env.example .env
# Edit .env with your Supabase details

# 2. Run migrations and start the server
cargo run -p app-api
```

### 7. Production Deployment

Use the updated deployment script:

```bash
# Set environment variables
export SUPABASE_URL="https://your-project.supabase.co"
export SUPABASE_ANON_KEY="your-anon-key" 
export SUPABASE_JWT_SECRET="your-jwt-secret"
export DATABASE_URL="your-supabase-database-url"

# Run deployment script
./scripts/deploy-flyio.sh
```

## Architecture Benefits

1. **Separation of Concerns**: 
   - Supabase handles authentication
   - App manages domain-specific data

2. **Flexibility**: 
   - Keep custom user profiles
   - Maintain existing business logic
   - Use Supabase for what it does best

3. **Security**:
   - Leverage Supabase's battle-tested auth
   - Implement RLS for fine-grained control

## Migration from Local to Supabase

If migrating from a local database:

1. Export existing data from your local database
2. Import into Supabase
3. Update configuration to use Supabase credentials
4. Test the application thoroughly

## Troubleshooting

### Common Issues

1. **Database Connection**: Ensure SSL is enabled in your connection string
2. **JWT Verification**: Make sure JWT_SECRET matches Supabase's JWT settings
3. **CORS**: Configure CORS settings in Supabase dashboard if needed

### Environment Variables

Make sure all required environment variables are set:
- `DATABASE_URL` - Supabase database connection string
- `SUPABASE_URL` - Your Supabase project URL 
- `SUPABASE_ANON_KEY` - Public API key
- `SUPABASE_JWT_SECRET` - JWT verification secret

## Scaling with Supabase

Supabase handles much of the scaling for you:
- Database scaling is managed by Supabase
- Authentication handles millions of users
- Real-time subscriptions can scale automatically

## Cost Considerations

Supabase's free tier includes:
- Up to 500MB storage
- Up to 100MB bandwidth
- Up to 10M monthly active users for auth

For larger applications, check Supabase's pricing page for appropriate plans.

## Security Best Practices

1. Never expose service_role_key on the frontend
2. Use Row Level Security for data access control
3. Keep JWT secret secure and rotate regularly
4. Use connection pooling appropriately