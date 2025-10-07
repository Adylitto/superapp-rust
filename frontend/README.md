# SuperApp Frontend

A Next.js frontend for the SuperApp - an all-in-one platform for social networking, ride sharing, payments, and DAO governance.

## 🚀 Quick Start

```bash
npm install
npm run dev
```

## 🏗️ Architecture

The frontend is built with:
- Next.js 14 with App Router
- TypeScript
- Tailwind CSS
- Framer Motion (for animations)
- React Icons
- React Query (for data fetching)

## 📁 Directory Structure

```
frontend/
├── public/           # Static assets
├── src/
│   ├── app/          # Next.js 14 App Router pages
│   ├── components/   # React components
│   ├── services/     # API services
│   ├── hooks/        # Custom hooks
│   └── styles/       # Global styles
```

## 🚀 Building for Production

```bash
npm run build
npm start  # or `npm run start` if you have a start script
```

## 🌐 Environment Variables

- `NEXT_PUBLIC_API_URL` - Backend API URL (defaults to http://localhost:8080)

## 🚀 Deployment

This app is configured for deployment to Vercel. When deploying:
1. Set the Root Directory to `frontend`
2. Set Build Command to `cd frontend && npm install && npm run build`
3. Set Output Directory to `.next`
4. Ensure Node.js version is set to 18.x or higher

## 🧪 Available Scripts

- `npm run dev` - Start development server
- `npm run build` - Build for production
- `npm run start` - Start production server
- `npm run type-check` - Run TypeScript type checking

## 🛠️ Configuration

- Next.js is configured in `next.config.js`
- Tailwind CSS is configured in `tailwind.config.ts`
- TypeScript is configured in `tsconfig.json`

## 🔐 API Integration

The frontend connects to the backend API via the `/src/services/api.ts` file. Update the `NEXT_PUBLIC_API_URL` environment variable to point to your backend service.