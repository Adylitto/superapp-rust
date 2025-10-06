# Connecting to Cloud Supabase

This guide explains how to connect your SuperApp to your cloud Supabase project.

## Step 1: Get Your Project Details

1. Go to https://app.supabase.com/
2. Select your project
3. Go to Project Settings → Database
4. Copy the following details:
   - **Connection string** (will be used as DATABASE_URL)
   - **Project URL** (used for SUPABASE_URL)
   - **Project ref** (needed for linking)

## Step 2: Update Environment Variables

Update your `.env` file with your actual Supabase cloud details:

```bash
# Database (Cloud Supabase)
# Replace with your actual connection string from Supabase dashboard
DATABASE_URL=postgresql://postgres:[YOUR_PASSWORD]@[YOUR_PROJECT_REF].supabase.co:5432/postgres

# Supabase Integration (Cloud)
SUPABASE_URL=https://[YOUR_PROJECT_REF].supabase.co
SUPABASE_ANON_KEY=your-anon-key-from-dashboard
SUPABASE_SERVICE_ROLE_KEY=your-service-role-key-from-dashboard
SUPABASE_JWT_SECRET=your-jwt-secret-from-dashboard
```

## Step 3: Link Your Project (Optional but Recommended)

To link your local project to your cloud Supabase project:

```bash
cd /Users/damani/Code/superapp-rust
supabase link --project-ref YOUR_PROJECT_REF
```

You can find your project ref in the Supabase dashboard URL:
`https://app.supabase.com/project/[YOUR_PROJECT_REF]`

## Step 4: Apply Database Migrations

Push your database schema to your cloud Supabase project:

```bash
cd /Users/damani/Code/superapp-rust
supabase db push
```

## Step 5: Test Database Connection

Test that your application can connect to your cloud Supabase database:

```bash
DATABASE_URL=postgresql://postgres:[YOUR_PASSWORD]@[YOUR_PROJECT_REF].supabase.co:5432/postgres cargo run -p app-storage --bin test-db
```

## Step 6: Run Your Application

Once everything is configured, run your application:

```bash
cargo run -p app-api
```

## Troubleshooting

### Connection Issues
1. Verify your database password is correct
2. Ensure your IP is allowed in Supabase database settings
3. Check that your project is not paused

### Authentication Issues
1. Make sure SUPABASE_ANON_KEY and SUPABASE_SERVICE_ROLE_KEY are correct
2. Verify SUPABASE_JWT_SECRET matches your project's JWT secret

### Migration Issues
1. Ensure your schema matches Supabase requirements
2. Check for any syntax differences between local and cloud PostgreSQL

## Supabase Dashboard Access

You can access your Supabase dashboard at:
https://app.supabase.com/project/[YOUR_PROJECT_REF]

From there you can:
- View and manage your database tables
- Monitor database performance
- Manage authentication settings
- Configure access policies
- View logs and analytics