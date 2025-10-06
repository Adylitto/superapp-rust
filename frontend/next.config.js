/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  images: {
    domains: ['localhost', 'lh3.googleusercontent.com'],
  },
  env: {
    API_URL: process.env.API_URL || process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8080',
  },
  // Vercel build optimization
  swcMinify: true,
  // Handle Vercel environment
  poweredByHeader: false,
  // Ensure proper asset handling
  assetPrefix: process.env.ASSET_PREFIX || '',
  // Handle trailing slashes consistently
  trailingSlash: false,
};

module.exports = nextConfig;
