# Vercel Deployment Fix Guide

This guide helps diagnose and fix common Vercel deployment issues for the SuperApp frontend.

## Common Deployment Issues and Solutions

### 1. Build Command Issues

**Problem**: Incorrect build command in `vercel.json`
**Solution**: Verify the build command matches your project structure:
```json
{
  "buildCommand": "cd frontend && npm install && npm run build",
  "outputDirectory": "frontend/.next"
}
```

### 2. Environment Variable Issues

**Problem**: Missing environment variables during build
**Solution**: 
1. Set environment variables in Vercel Dashboard → Project Settings → Environment Variables
2. Use `vercel env pull` to sync local environment variables

### 3. Large File Issues

**Problem**: Large cache files exceeding Vercel limits
**Solution**: Add cache directories to `.vercelignore`:
```
# Next.js cache
.next/cache/
```

### 4. Node.js Version Mismatch

**Problem**: Different Node.js versions between local and Vercel
**Solution**: Specify Node.js version in `package.json`:
```json
{
  "engines": {
    "node": "18.x"
  }
}
```

### 5. Dependency Issues

**Problem**: Missing or incorrect dependencies
**Solution**: 
1. Run `npm install` locally to ensure `package-lock.json` is up to date
2. Clear Vercel cache and redeploy

## Diagnostic Commands

### Check Local Build
```bash
cd frontend
npm run build
```

### Check TypeScript
```bash
cd frontend
npm run type-check
```

### Check Vercel Configuration
```bash
npx vercel env pull
```

## Manual Deployment

To manually deploy to Vercel:
```bash
cd /Users/damani/Code/superapp-rust
npx vercel --prod
```

## Troubleshooting Steps

### 1. Clear Build Cache
```bash
# In Vercel Dashboard:
# Project Settings → Git → Deploy Hooks → Redeploy without cache
```

### 2. Check Build Logs
```bash
# In Vercel Dashboard:
# Latest Deployment → View Logs
```

### 3. Verify Environment Variables
```bash
# In Vercel Dashboard:
# Project Settings → Environment Variables
# Make sure all required variables are set
```

### 4. Check Domain Configuration
```bash
# In Vercel Dashboard:
# Domains → Verify custom domains are properly configured
```

## Common Error Messages and Fixes

### "Module not found"
- **Cause**: Missing dependencies
- **Fix**: Run `npm install` and commit updated `package-lock.json`

### "Build exceeded 15 minute timeout"
- **Cause**: Large project or slow dependencies
- **Fix**: Optimize dependencies, use incremental builds

### "Out of memory"
- **Cause**: Large build process
- **Fix**: Reduce bundle size, optimize images, use code splitting

### "Failed to compile"
- **Cause**: TypeScript or syntax errors
- **Fix**: Run `npm run type-check` locally to identify issues

## Best Practices

1. **Keep dependencies minimal**
2. **Optimize images and assets**
3. **Use code splitting for large bundles**
4. **Set appropriate environment variables**
5. **Monitor build times and sizes**
6. **Use Vercel's caching mechanisms**

## Vercel CLI Commands

```bash
# Deploy to production
npx vercel --prod

# Deploy to preview
npx vercel

# Pull environment variables
npx vercel env pull

# Check deployment status
npx vercel inspect

# View logs
npx vercel logs [deployment-url]
```

## Contact Support

If issues persist:
1. Check [Vercel Documentation](https://vercel.com/docs)
2. Visit [Vercel Community](https://github.com/vercel/community/discussions)
3. Contact [Vercel Support](https://vercel.com/support)