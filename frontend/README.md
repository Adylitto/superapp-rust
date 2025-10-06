# SuperApp Frontend

Modern, beautiful, and blazingly fast frontend for the SuperApp platform.

## 🎨 Tech Stack

- **Framework**: Next.js 14 (App Router)
- **Language**: TypeScript
- **Styling**: Tailwind CSS
- **State Management**: Zustand
- **Data Fetching**: TanStack Query (React Query)
- **Animations**: Framer Motion
- **Icons**: React Icons
- **HTTP Client**: Axios

## 🚀 Quick Start

### Installation

```bash
cd frontend

# Install dependencies
npm install

# Copy environment file
cp .env.local.example .env.local

# Start development server
npm run dev
```

Frontend will be available at `http://localhost:3000`

Make sure your backend API is running at `http://localhost:8080`

### Environment Variables

```env
# .env.local
NEXT_PUBLIC_API_URL=http://localhost:8080
NEXT_PUBLIC_WS_URL=ws://localhost:8080/ws
```

## 📱 Features

### ✅ Landing Page
- Beautiful gradient hero section
- Feature cards with animations
- Stats showcase
- Call-to-action sections
- Fully responsive design

### 🎯 Planned Features
- [ ] Authentication (Login/Register)
- [ ] Social Feed with infinite scroll
- [ ] Real-time messaging
- [ ] Ride-sharing interface with maps
- [ ] Token wallet and transactions
- [ ] DAO governance dashboard
- [ ] Mini-apps marketplace

## 🎨 Design System

### Colors
```css
Primary: Blue (#0ea5e9)
Secondary: Purple (#a855f7)
Success: Green (#10b981)
Warning: Orange (#f59e0b)
Error: Red (#ef4444)
```

### Components

#### Buttons
```tsx
<button className="btn-primary">Primary Button</button>
<button className="btn-secondary">Secondary Button</button>
```

#### Cards
```tsx
<div className="card">
  <h3>Card Title</h3>
  <p>Card content</p>
</div>
```

#### Input Fields
```tsx
<input className="input-field" placeholder="Enter text" />
```

## 📂 Project Structure

```
frontend/
├── public/              # Static assets
├── src/
│   ├── app/            # Next.js app router pages
│   │   ├── layout.tsx  # Root layout with providers
│   │   ├── page.tsx    # Landing page
│   │   └── globals.css # Global styles
│   ├── components/     # Reusable UI components
│   │   └── Navbar.tsx  # Navigation component
│   ├── services/       # API service layer
│   │   └── api.ts      # Axios client & API calls
│   ├── hooks/          # Custom React hooks
│   ├── utils/          # Helper functions
│   ├── assets/         # Images, fonts, etc.
│   └── styles/         # Additional styles
├── package.json
├── tsconfig.json
├── tailwind.config.ts
└── next.config.js
```

## 🔧 Development

### Available Scripts

```bash
# Development server
npm run dev

# Production build
npm run build

# Start production server
npm start

# Linting
npm run lint

# Type checking
npm run type-check
```

### Adding New Pages

1. Create file in `src/app/[route]/page.tsx`
2. Use the App Router conventions
3. Add to navigation in `Navbar.tsx`

Example:
```tsx
// src/app/social/page.tsx
export default function SocialPage() {
  return (
    <div>
      <h1>Social Feed</h1>
    </div>
  );
}
```

### API Integration

```typescript
import { socialAPI } from '@/services/api';

// Create a post
const handlePost = async () => {
  try {
    const result = await socialAPI.createPost('Hello World!');
    console.log('Post created:', result);
  } catch (error) {
    console.error('Error:', error);
  }
};
```

### State Management

```typescript
import create from 'zustand';

interface UserStore {
  user: User | null;
  setUser: (user: User) => void;
}

export const useUserStore = create<UserStore>((set) => ({
  user: null,
  setUser: (user) => set({ user }),
}));
```

## 🎭 Animations

Using Framer Motion:

```tsx
import { motion } from 'framer-motion';

<motion.div
  initial={{ opacity: 0, y: 20 }}
  animate={{ opacity: 1, y: 0 }}
  transition={{ duration: 0.5 }}
>
  Animated content
</motion.div>
```

## 📱 Responsive Design

Mobile-first approach with Tailwind breakpoints:

```tsx
<div className="
  grid grid-cols-1
  md:grid-cols-2
  lg:grid-cols-3
  gap-4
">
  {/* Responsive grid */}
</div>
```

## 🚀 Deployment

### Vercel (Recommended)

```bash
# Install Vercel CLI
npm i -g vercel

# Deploy
vercel
```

### Docker

```bash
# Build image
docker build -t superapp-frontend .

# Run container
docker run -p 3000:3000 superapp-frontend
```

### Environment Variables

Set these in your deployment platform:
- `NEXT_PUBLIC_API_URL`
- `NEXT_PUBLIC_WS_URL`

## 🎯 Performance

- **Code Splitting**: Automatic with Next.js
- **Image Optimization**: Next.js Image component
- **Bundle Size**: Monitored with `next build`
- **Caching**: TanStack Query for API calls

## 🔐 Security

- HTTPS in production
- XSS protection via React
- CSRF tokens for API calls
- Secure token storage
- Input sanitization

## 📚 Resources

- [Next.js Docs](https://nextjs.org/docs)
- [Tailwind CSS](https://tailwindcss.com/docs)
- [Framer Motion](https://www.framer.com/motion/)
- [TanStack Query](https://tanstack.com/query/latest)

## 🤝 Contributing

1. Create feature branch
2. Make changes
3. Test thoroughly
4. Submit pull request

## 📄 License

MIT OR Apache-2.0

---

**Built with ❤️ using Next.js and TypeScript**
