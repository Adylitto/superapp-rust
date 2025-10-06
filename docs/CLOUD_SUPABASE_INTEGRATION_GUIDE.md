# 🌐 SuperApp Cloud Supabase Integration Guide

This guide explains how to connect your SuperApp to a cloud Supabase project for production deployment.

## 📋 Prerequisites

1. A Supabase account (https://supabase.com)
2. A Supabase project created
3. Supabase CLI installed (`brew install supabase/tap/supabase`)
4. Your SuperApp codebase

## 🚀 Quick Setup

Run the automated setup script:
```bash
cd /Users/damani/Code/superapp-rust
./scripts/connect-cloud-supabase.sh
```

Follow the prompts to enter your Supabase project details.

## 📝 Manual Setup

### Step 1: Get Your Supabase Project Details

1. Go to https://app.supabase.com/
2. Select your project
3. Go to Project Settings → Database
4. Note down:
   - **Project URL**: `https://[YOUR_PROJECT_REF].supabase.co`
   - **Database Connection String**: `postgresql://postgres:[YOUR_PASSWORD]@[YOUR_PROJECT_REF].supabase.co:5432/postgres`
5. Go to Project Settings → API
6. Note down:
   - **Anonymous Key**: For frontend use
   - **Service Role Key**: For backend use

### Step 2: Update Environment Variables

Update your `.env` file with your actual Supabase details:

```bash
# Database (Cloud Supabase)
DATABASE_URL=postgresql://postgres:YOUR_ACTUAL_PASSWORD@YOUR_PROJECT_REF.supabase.co:5432/postgres
DATABASE_MAX_CONNECTIONS=100
DATABASE_MIN_CONNECTIONS=5

# Supabase Integration (Cloud)
SUPABASE_URL=https://YOUR_PROJECT_REF.supabase.co
SUPABASE_ANON_KEY=your-actual-anon-key
SUPABASE_SERVICE_ROLE_KEY=your-actual-service-role-key
SUPABASE_JWT_SECRET=your-actual-jwt-secret
```

### Step 3: Apply Database Schema

Apply your database schema to your cloud Supabase project:

```bash
cd /Users/damani/Code/superapp-rust
supabase db push
```

If that doesn't work, manually apply the schema:
1. Go to your Supabase dashboard
2. Navigate to SQL Editor
3. Copy and paste the contents of `crates/app-storage/migrations/001_initial_schema.sql`
4. Run the SQL to create your tables

### Step 4: Test Database Connection

Test that your application can connect to your cloud Supabase database:

```bash
DATABASE_URL=postgresql://postgres:YOUR_PASSWORD@YOUR_PROJECT_REF.supabase.co:5432/postgres \
cargo run -p app-storage --bin test-db
```

### Step 5: Run Your Application

Once everything is configured, run your application:

```bash
cargo run -p app-api
```

## 🌐 Vercel Deployment with Cloud Supabase

### Update Vercel Configuration

Make sure your `vercel.json` is configured correctly:

```json
{
  "$schema": "https://openapi.vercel.sh/vercel.json",
  "version": 2,
  "buildCommand": "cd frontend && npm install && npm run build",
  "devCommand": "cd frontend && npm run dev",
  "installCommand": "cd frontend && npm install",
  "framework": "nextjs",
  "outputDirectory": "frontend/.next",
  "github": {
    "silent": true
  },
  "regions": ["iad1"]
}
```

### Set Environment Variables in Vercel

In your Vercel dashboard:
1. Go to your project settings
2. Navigate to Environment Variables
3. Add the following variables:
   - `DATABASE_URL` - Your Supabase connection string
   - `SUPABASE_URL` - Your Supabase project URL
   - `SUPABASE_ANON_KEY` - Your anonymous key
   - Any other required environment variables

## 🔧 Troubleshooting

### Database Connection Issues

1. **Verify Connection String**: Make sure your DATABASE_URL is correct
2. **Check Password**: Ensure your database password is correct
3. **IP Restrictions**: Check if your IP is allowed in Supabase database settings
4. **SSL Requirements**: Cloud Supabase requires SSL connections

### Authentication Issues

1. **API Keys**: Make sure you're using the correct Anonymous Key and Service Role Key
2. **JWT Secret**: Ensure SUPABASE_JWT_SECRET matches your project's JWT secret

### Migration Issues

1. **Schema Compatibility**: Ensure your schema is compatible with Supabase PostgreSQL
2. **Permissions**: Make sure your database user has the necessary permissions

## 🔒 Security Best Practices

1. **Never commit secrets**: Keep all API keys and passwords in environment variables
2. **Use different keys**: Use Anonymous Key for frontend, Service Role Key for backend
3. **Rotate secrets regularly**: Change your keys periodically
4. **Enable Row Level Security**: Implement RLS policies in Supabase for data protection
5. **Use connection pooling**: Configure appropriate connection limits

## 📊 Monitoring and Maintenance

### Database Monitoring

1. **Performance**: Monitor query performance in Supabase dashboard
2. **Connections**: Watch connection usage to avoid hitting limits
3. **Storage**: Monitor storage usage and plan accordingly

### Backups

1. **Point-in-time Recovery**: Supabase automatically provides PITR
2. **Manual Backups**: Create manual backups before major changes
3. **Export Data**: Regularly export important data

## 🚀 Production Deployment Checklist

- [ ] Database connection tested and working
- [ ] Environment variables properly configured
- [ ] Database schema applied to production
- [ ] API keys and secrets secured
- [ ] Connection pooling configured
- [ ] Error handling and logging implemented
- [ ] Performance monitoring set up
- [ ] Backup strategy in place
- [ ] Security policies reviewed
- [ ] Rate limiting configured
- [ ] SSL/TLS enforced

## 🆘 Support

If you encounter issues:

1. Check the [Supabase Documentation](https://supabase.com/docs)
2. Visit the [Supabase Community Forum](https://github.com/supabase/supabase/discussions)
3. Contact [Supabase Support](https://supabase.com/support)
4. Refer to the SuperApp documentation in `/docs`

## 📚 Additional Resources

- [Supabase Official Documentation](https://supabase.com/docs)
- [Next.js with Supabase Guide](https://supabase.com/docs/guides/getting-started/tutorials/with-nextjs)
- [Supabase Authentication](https://supabase.com/docs/guides/auth)
- [Supabase Database](https://supabase.com/docs/guides/database)
- [Supabase Realtime](https://supabase.com/docs/guides/realtime)