# 🚀 SuperApp Frontend Deployment Guide

This document explains how to deploy the SuperApp frontend to Vercel.

## 📋 Deployment Prerequisites

1. A Vercel account (https://vercel.com)
2. The Vercel CLI installed (`npm i -g vercel`)
3. Your SuperApp frontend codebase

## 🔧 Vercel Project Configuration - CRITICAL STEP

For the frontend to deploy correctly to Vercel, you **MUST** configure your Vercel project settings properly:

### Configure Project Settings in Vercel Dashboard (REQUIRED)

1. Go to your Vercel Dashboard: https://vercel.com/dashboard
2. Select your `superapp-rust` project
3. Go to Settings → General
4. Change the **Root Directory** to `frontend`
5. Save the changes
6. Redeploy your project

**This step is required because your Next.js frontend is in the `/frontend` subdirectory, not in the root of your repository.**

### Screenshot Guide:
- Project Settings → General Settings → Build & Development Settings → Root Directory
- Enter: `frontend`
- Click Save

## 🏗️ Vercel Build Configuration

The project is set up with the following configuration in `vercel.json`:

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

## 🌐 Environment Variables

Make sure to set these environment variables in your Vercel project:

- `NEXT_PUBLIC_API_URL` - Your backend API URL (e.g., `https://your-backend-app.fly.dev`)

To add environment variables:
1. Go to your Vercel project dashboard
2. Navigate to Settings → Environment Variables
3. Add the required variables

## 🚀 Deployment Commands

### Deploy to Preview
```bash
cd /Users/damani/Code/superapp-rust
npx vercel
```

### Deploy to Production
```bash
cd /Users/damani/Code/superapp-rust
npx vercel --prod
```

## 🔍 Troubleshooting

### Common Issues:

1. **"No Next.js version detected"** (YOUR CURRENT ERROR):
   - This happens when the Root Directory is NOT set to `frontend`
   - **Solution**: Go to Vercel dashboard → Project Settings → General → Root Directory and set it to `frontend`
   - Verify that `package.json` exists in the `frontend/` directory
   - Check that `next` is listed in dependencies in `frontend/package.json`

2. **Build fails**:
   - Check that the build command is `cd frontend && npm install && npm run build`
   - Verify that the output directory is `frontend/.next`

3. **404 errors after deployment**:
   - Ensure the framework is set to Next.js
   - Check that the root directory setting is correct

## 🧪 Testing Your Deployment

After deployment, you can test your deployed frontend:

1. Visit the URL provided by Vercel after deployment
2. Check that all pages load correctly
3. Test the login and registration flows
4. Verify that the API calls work (when backend is connected)

## 🔄 Redeploying After Changes

After making changes to your frontend code:

1. Commit your changes to Git
2. Push to your connected Git repository (if using Git integration), or
3. Run `npx vercel --prod` to deploy manually

## 📞 Support

If you continue to experience deployment issues:

1. Check the Vercel logs in your dashboard
2. Visit the Vercel documentation: https://vercel.com/docs
3. Reach out to Vercel support if needed

## ⚠️ CRITICAL FIX FOR YOUR CURRENT ERROR

Your current deployment is failing with "No Next.js version detected". This is because:

1. Your Next.js project is in `/frontend` directory
2. But Vercel is looking for it in the root directory
3. You need to set the Root Directory to `frontend` in the Vercel dashboard

**Please follow the steps in the "Vercel Project Configuration" section above before attempting to deploy again.**

Once you set the Root Directory in the dashboard, your deployments will work correctly.