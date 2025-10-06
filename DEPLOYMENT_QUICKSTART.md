# 🚀 SuperApp Deployment Quick Start

## Current Status

✅ **Frontend**: Deployed to Vercel
🔄 **Backend**: Ready for Fly.io deployment
⏳ **Database**: Will be created during deployment

## Deploy Backend to Fly.io

### Option 1: Automated (Recommended)

```bash
cd /Users/damani/Code/superapp-rust
./scripts/deploy-flyio.sh
```

This script handles everything automatically!

### Option 2: Manual Steps

```bash
# 1. Add flyctl to PATH
export FLYCTL_INSTALL="/Users/damani/.fly"
export PATH="$FLYCTL_INSTALL/bin:$PATH"

# 2. Login to Fly.io
flyctl auth login

# 3. Deploy (creates app, database, and deploys)
flyctl launch --name superapp-rust-api --region iad

# 4. Set secrets
JWT_SECRET=$(openssl rand -base64 32)
flyctl secrets set \
    JWT_SECRET="$JWT_SECRET" \
    RUST_LOG="info" \
    --app superapp-rust-api
```

## After Backend Deployment

### 1. Get Your API URL

```bash
flyctl apps list | grep superapp-rust-api
# Example output: https://superapp-rust-api.fly.dev
```

### 2. Update Frontend Environment

Update Vercel environment variables:

```bash
# Go to: https://vercel.com/adyls-projects-b070f026/frontend/settings/environment-variables

# Set:
NEXT_PUBLIC_API_URL=https://superapp-rust-api.fly.dev
```

### 3. Redeploy Frontend

```bash
cd frontend
vercel --prod
```

## Verify Deployment

```bash
# Check backend health
curl https://superapp-rust-api.fly.dev/health

# Check frontend
open https://frontend-1gi1v4oel-adyls-projects-b070f026.vercel.app

# Try registration
# Should connect to backend API
```

## Useful Commands

```bash
# View backend logs
flyctl logs --app superapp-rust-api

# Check backend status
flyctl status --app superapp-rust-api

# SSH into backend
flyctl ssh console --app superapp-rust-api

# Connect to database
flyctl postgres connect --app superapp-db

# View frontend logs (Vercel)
vercel logs https://frontend-1gi1v4oel-adyls-projects-b070f026.vercel.app
```

## Costs

### Fly.io (Backend)
- **Free Tier**: Covers small apps
- **Estimated**: $0-10/month for MVP

### Vercel (Frontend)
- **Free Tier**: Covers hobby projects
- **Estimated**: $0/month for MVP

### Total: ~$0-10/month for MVP

## Troubleshooting

### Backend won't start
```bash
flyctl logs --app superapp-rust-api
# Check for database connection issues
```

### Frontend can't reach backend
1. Check `NEXT_PUBLIC_API_URL` is set correctly
2. Check CORS settings in backend
3. Verify backend is running: `curl https://superapp-rust-api.fly.dev/health`

### Database connection failed
```bash
# Check DATABASE_URL is set
flyctl secrets list --app superapp-rust-api

# Test connection
flyctl postgres connect --app superapp-db
```

## Next Steps

After successful deployment:

1. ✅ Test registration flow
2. ✅ Test login flow
3. ✅ Verify token authentication
4. 🔄 Set up monitoring (Sentry, DataDog)
5. 🔄 Configure custom domain
6. 🔄 Set up CI/CD pipeline
7. 🔄 Run database migrations
8. 🔄 Deploy Solana smart contracts

## Documentation

- **Fly.io Guide**: `/docs/FLYIO_DEPLOYMENT.md`
- **General Deployment**: `/docs/DEPLOYMENT.md`
- **Frontend README**: `/frontend/README.md`
- **Main README**: `/README.md`

---

**Ready to deploy?** Run: `./scripts/deploy-flyio.sh`
