# SuperApp Deployment Guide

## 🚀 Frontend Deployment

### ✅ Successfully Deployed

**Production URL**: https://frontend-1gi1v4oel-adyls-projects-b070f026.vercel.app

**Platform**: Vercel
**Status**: Live and running
**Build**: Successful
**Framework**: Next.js 14.2.0

### Pages Available

1. **Landing Page** - `/`
   - Beautiful hero section with gradients
   - Feature cards with animations
   - Stats showcase
   - CTA sections

2. **Login Page** - `/login`
   - Email and password authentication
   - Form validation
   - Social login buttons (Google, GitHub)
   - Error handling

3. **Register Page** - `/register`
   - User registration form
   - Password strength indicator
   - Terms acceptance
   - Token rewards preview

4. **Protected Routes** (require authentication)
   - `/social` - Social feed
   - `/messages` - Messaging
   - `/rides` - Ride sharing
   - `/wallet` - Token wallet
   - `/dao` - DAO governance
   - `/apps` - Mini apps

### Build Statistics

```
Route (app)                              Size     First Load JS
┌ ○ /                                    1.84 kB         133 kB
├ ○ /_not-found                          871 B          88.3 kB
├ ○ /login                               2.9 kB          156 kB
└ ○ /register                            3.54 kB         157 kB
+ First Load JS shared by all            87.4 kB
ƒ Middleware                             27 kB
```

### Environment Variables (Vercel)

Set these in your Vercel project settings:

```env
NEXT_PUBLIC_API_URL=https://your-backend-url.com
NEXT_PUBLIC_WS_URL=wss://your-backend-url.com/ws
```

**Current Configuration**:
- `NEXT_PUBLIC_API_URL`: http://localhost:8080 (for local development)

### Deployment Commands

```bash
# Deploy to production
vercel --prod

# Deploy to preview
vercel

# View deployment logs
vercel inspect <deployment-url> --logs

# Redeploy
vercel redeploy <deployment-url>
```

## 🖥️ Backend Deployment (Pending)

### Options for Backend Deployment

#### 1. Fly.io (Recommended for Rust)

```bash
# Install flyctl
curl -L https://fly.io/install.sh | sh

# Login
flyctl auth login

# Create app
flyctl launch

# Deploy
flyctl deploy
```

**Dockerfile** (create in project root):

```dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release -p app-api

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y \
    libpq-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/app-api /usr/local/bin/app-api
ENV RUST_LOG=info
EXPOSE 8080
CMD ["app-api"]
```

#### 2. Railway

```bash
# Install Railway CLI
npm i -g @railway/cli

# Login
railway login

# Initialize project
railway init

# Deploy
railway up
```

#### 3. AWS ECS (Production-grade)

- Use AWS Fargate for containerized deployment
- Set up RDS PostgreSQL for database
- ElastiCache Redis for caching
- ALB for load balancing

#### 4. DigitalOcean App Platform

- Connect GitHub repository
- Automatic builds and deployments
- Managed PostgreSQL database
- Built-in monitoring

### Required Services

1. **PostgreSQL Database**
   - Vercel Postgres
   - Supabase
   - Railway PostgreSQL
   - AWS RDS

2. **Redis Cache** (optional)
   - Upstash Redis
   - Railway Redis
   - AWS ElastiCache

3. **Environment Variables**

```env
DATABASE_URL=postgresql://user:pass@host:5432/dbname
REDIS_URL=redis://host:6379
JWT_SECRET=your-secret-key
RUST_LOG=info
```

## 📦 Solana Smart Contract Deployment

### Prerequisites

```bash
# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/v1.17.0/install)"

# Install Anchor
cargo install --git https://github.com/coral-xyz/anchor avm --locked
avm install latest
avm use latest
```

### Deploy to Devnet

```bash
cd app-dao

# Configure Solana CLI for devnet
solana config set --url devnet

# Create keypair (if needed)
solana-keygen new

# Airdrop SOL for deployment
solana airdrop 2

# Build program
anchor build

# Deploy
anchor deploy
```

### Deploy to Mainnet

```bash
# Configure for mainnet
solana config set --url mainnet-beta

# Deploy (requires SOL in wallet)
anchor deploy
```

## 🔄 CI/CD Pipeline

### GitHub Actions (Recommended)

Create `.github/workflows/deploy.yml`:

```yaml
name: Deploy

on:
  push:
    branches: [main]

jobs:
  frontend:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Deploy to Vercel
        uses: amondnet/vercel-action@v25
        with:
          vercel-token: ${{ secrets.VERCEL_TOKEN }}
          vercel-org-id: ${{ secrets.VERCEL_ORG_ID }}
          vercel-project-id: ${{ secrets.VERCEL_PROJECT_ID }}

  backend:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Build and deploy
        run: |
          # Add your backend deployment commands
```

## 🔐 Security Checklist

- [ ] Environment variables configured securely
- [ ] CORS configured for production domains
- [ ] Rate limiting enabled
- [ ] HTTPS enforced
- [ ] JWT secrets rotated regularly
- [ ] Database backups configured
- [ ] Monitoring and alerts set up
- [ ] API keys stored in secrets manager

## 📊 Monitoring

### Frontend (Vercel)

- Built-in analytics
- Web Vitals monitoring
- Error tracking
- Deployment logs

### Backend (Recommended Tools)

- **Sentry**: Error tracking
- **DataDog**: APM and logging
- **Prometheus + Grafana**: Metrics
- **CloudWatch**: AWS infrastructure

## 🚨 Rollback Procedure

### Frontend (Vercel)

```bash
# List deployments
vercel ls

# Promote previous deployment
vercel promote <deployment-url>
```

### Backend

```bash
# Depends on platform
flyctl apps releases
flyctl releases rollback <version>
```

## 📝 Post-Deployment Checklist

### Frontend
- ✅ Landing page loads correctly
- ✅ Login/register pages functional
- ✅ Protected routes redirect properly
- ✅ API integration working
- ✅ Responsive design verified
- ⏳ Update API URL to production backend
- ⏳ Configure custom domain
- ⏳ Set up monitoring

### Backend
- ⏳ Deploy API server
- ⏳ Database migrations run successfully
- ⏳ Health check endpoint responding
- ⏳ JWT authentication working
- ⏳ Rate limiting configured
- ⏳ CORS configured for frontend domain
- ⏳ WebSocket connections working
- ⏳ Background jobs running

### Smart Contracts
- ⏳ Deploy to Solana devnet
- ⏳ Test token minting
- ⏳ Test DAO governance
- ⏳ Verify on Solana Explorer
- ⏳ Deploy to mainnet

## 🔗 Useful Links

- **Frontend**: https://frontend-1gi1v4oel-adyls-projects-b070f026.vercel.app
- **Vercel Dashboard**: https://vercel.com/adyls-projects-b070f026/frontend
- **GitHub Repository**: https://github.com/Adylitto/superapp-rust

## 🆘 Troubleshooting

### Frontend Build Fails

1. Check `package.json` dependencies
2. Run `npm run build` locally
3. Check Vercel build logs
4. Verify environment variables

### Backend Deployment Issues

1. Check Docker build logs
2. Verify database connection
3. Check environment variables
4. Review application logs

### Database Connection Errors

1. Verify `DATABASE_URL` format
2. Check firewall rules
3. Ensure database is running
4. Test connection with `psql`

---

**Last Updated**: 2025-10-06
**Status**: Frontend deployed, backend pending
