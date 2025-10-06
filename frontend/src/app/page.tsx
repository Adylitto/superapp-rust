'use client';

import { motion } from 'framer-motion';
import Link from 'next/link';
import { FiUser, FiMessageCircle, FiTruck, FiDollarSign, FiBox, FiZap } from 'react-icons/fi';

const features = [
  {
    icon: FiUser,
    title: 'Social Network',
    description: 'Connect with friends, share posts, and discover new connections',
    color: 'from-blue-500 to-cyan-500',
    href: '/social',
  },
  {
    icon: FiMessageCircle,
    title: 'Real-Time Chat',
    description: 'End-to-end encrypted messaging with real-time updates',
    color: 'from-green-500 to-emerald-500',
    href: '/messages',
  },
  {
    icon: FiTruck,
    title: 'Ride Sharing',
    description: 'Request rides with AI-optimized routing and driver matching',
    color: 'from-purple-500 to-pink-500',
    href: '/rides',
  },
  {
    icon: FiDollarSign,
    title: 'Payments',
    description: 'Seamless fiat and crypto payments with built-in wallet',
    color: 'from-yellow-500 to-orange-500',
    href: '/wallet',
  },
  {
    icon: FiBox,
    title: 'DAO Governance',
    description: 'Participate in decentralized decision-making with token voting',
    color: 'from-indigo-500 to-purple-500',
    href: '/dao',
  },
  {
    icon: FiZap,
    title: 'Mini Apps',
    description: 'Access third-party services and earn tokens for engagement',
    color: 'from-red-500 to-pink-500',
    href: '/apps',
  },
];

export default function Home() {
  return (
    <div className="min-h-screen">
      {/* Hero Section */}
      <section className="gradient-bg text-white py-20 px-4">
        <div className="container mx-auto max-w-6xl">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.6 }}
            className="text-center"
          >
            <h1 className="text-5xl md:text-7xl font-bold mb-6">
              Welcome to SuperApp
            </h1>
            <p className="text-xl md:text-2xl mb-8 text-white/90">
              Your all-in-one platform for social, mobility, payments, and governance
            </p>
            <div className="flex gap-4 justify-center flex-wrap">
              <Link href="/register">
                <motion.button
                  whileHover={{ scale: 1.05 }}
                  whileTap={{ scale: 0.95 }}
                  className="bg-white text-primary-600 px-8 py-4 rounded-full font-bold text-lg shadow-lg hover:shadow-xl transition-all"
                >
                  Get Started
                </motion.button>
              </Link>
              <Link href="/login">
                <motion.button
                  whileHover={{ scale: 1.05 }}
                  whileTap={{ scale: 0.95 }}
                  className="bg-white/10 backdrop-blur-sm text-white px-8 py-4 rounded-full font-bold text-lg border-2 border-white/30 hover:bg-white/20 transition-all"
                >
                  Sign In
                </motion.button>
              </Link>
            </div>
          </motion.div>
        </div>
      </section>

      {/* Features Grid */}
      <section className="py-20 px-4">
        <div className="container mx-auto max-w-6xl">
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            transition={{ delay: 0.3 }}
            className="text-center mb-16"
          >
            <h2 className="text-4xl md:text-5xl font-bold mb-4">
              Everything You Need
            </h2>
            <p className="text-xl text-gray-600">
              Powered by Rust, secured by blockchain, enhanced by AI
            </p>
          </motion.div>

          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
            {features.map((feature, index) => (
              <motion.div
                key={feature.title}
                initial={{ opacity: 0, y: 20 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ delay: 0.1 * index }}
                whileHover={{ y: -8, scale: 1.02 }}
                className="group"
              >
                <Link href={feature.href}>
                  <div className="card h-full cursor-pointer">
                    <div className={`w-14 h-14 rounded-xl bg-gradient-to-br ${feature.color} flex items-center justify-center mb-4 group-hover:scale-110 transition-transform`}>
                      <feature.icon className="text-2xl text-white" />
                    </div>
                    <h3 className="text-2xl font-bold mb-2 group-hover:text-primary-600 transition-colors">
                      {feature.title}
                    </h3>
                    <p className="text-gray-600">
                      {feature.description}
                    </p>
                  </div>
                </Link>
              </motion.div>
            ))}
          </div>
        </div>
      </section>

      {/* Stats Section */}
      <section className="bg-gray-100 py-20 px-4">
        <div className="container mx-auto max-w-6xl">
          <div className="grid grid-cols-2 md:grid-cols-4 gap-8 text-center">
            {[
              { label: 'Active Users', value: '10K+' },
              { label: 'Rides Completed', value: '50K+' },
              { label: 'Tokens Earned', value: '1M+' },
              { label: 'DAO Proposals', value: '500+' },
            ].map((stat, index) => (
              <motion.div
                key={stat.label}
                initial={{ opacity: 0, scale: 0.5 }}
                animate={{ opacity: 1, scale: 1 }}
                transition={{ delay: 0.1 * index }}
              >
                <div className="text-4xl md:text-5xl font-bold text-primary-600 mb-2">
                  {stat.value}
                </div>
                <div className="text-gray-600 font-medium">
                  {stat.label}
                </div>
              </motion.div>
            ))}
          </div>
        </div>
      </section>

      {/* CTA Section */}
      <section className="py-20 px-4">
        <div className="container mx-auto max-w-4xl">
          <motion.div
            initial={{ opacity: 0, scale: 0.9 }}
            animate={{ opacity: 1, scale: 1 }}
            className="gradient-bg text-white rounded-3xl p-12 text-center shadow-2xl"
          >
            <h2 className="text-4xl md:text-5xl font-bold mb-4">
              Ready to Join?
            </h2>
            <p className="text-xl mb-8 text-white/90">
              Start earning tokens for your engagement today
            </p>
            <Link href="/register">
              <motion.button
                whileHover={{ scale: 1.05 }}
                whileTap={{ scale: 0.95 }}
                className="bg-white text-primary-600 px-10 py-4 rounded-full font-bold text-lg shadow-lg hover:shadow-xl transition-all"
              >
                Create Free Account
              </motion.button>
            </Link>
          </motion.div>
        </div>
      </section>

      {/* Footer */}
      <footer className="bg-gray-900 text-white py-12 px-4">
        <div className="container mx-auto max-w-6xl text-center">
          <p className="text-gray-400">
            © 2025 SuperApp. Built with Rust 🦀 · Powered by Blockchain · Enhanced by AI
          </p>
        </div>
      </footer>
    </div>
  );
}
