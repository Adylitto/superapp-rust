'use client';

import Link from 'next/link';
import { FiHome, FiUsers, FiMessageCircle, FiTruck, FiDollarSign, FiBox } from 'react-icons/fi';
import { motion } from 'framer-motion';

const navItems = [
  { icon: FiHome, label: 'Home', href: '/' },
  { icon: FiUsers, label: 'Social', href: '/social' },
  { icon: FiMessageCircle, label: 'Messages', href: '/messages' },
  { icon: FiTruck, label: 'Rides', href: '/rides' },
  { icon: FiDollarSign, label: 'Wallet', href: '/wallet' },
  { icon: FiBox, label: 'DAO', href: '/dao' },
];

export default function Navbar() {
  return (
    <nav className="bg-white shadow-sm border-b border-gray-200 sticky top-0 z-50">
      <div className="container mx-auto px-4">
        <div className="flex items-center justify-between h-16">
          {/* Logo */}
          <Link href="/">
            <motion.div
              whileHover={{ scale: 1.05 }}
              className="text-2xl font-bold gradient-bg bg-clip-text text-transparent cursor-pointer"
            >
              SuperApp
            </motion.div>
          </Link>

          {/* Navigation */}
          <div className="hidden md:flex items-center space-x-1">
            {navItems.map((item) => (
              <Link key={item.href} href={item.href}>
                <motion.div
                  whileHover={{ scale: 1.05 }}
                  whileTap={{ scale: 0.95 }}
                  className="flex items-center gap-2 px-4 py-2 rounded-lg hover:bg-gray-100 transition-colors cursor-pointer"
                >
                  <item.icon className="text-lg" />
                  <span className="font-medium">{item.label}</span>
                </motion.div>
              </Link>
            ))}
          </div>

          {/* User Menu */}
          <div className="flex items-center gap-4">
            <Link href="/login">
              <button className="btn-secondary">
                Sign In
              </button>
            </Link>
            <Link href="/register">
              <button className="btn-primary">
                Sign Up
              </button>
            </Link>
          </div>
        </div>

        {/* Mobile Navigation */}
        <div className="md:hidden flex justify-around py-2 border-t border-gray-200">
          {navItems.map((item) => (
            <Link key={item.href} href={item.href}>
              <motion.div
                whileTap={{ scale: 0.9 }}
                className="flex flex-col items-center gap-1 p-2 rounded-lg hover:bg-gray-100 transition-colors"
              >
                <item.icon className="text-xl" />
                <span className="text-xs">{item.label}</span>
              </motion.div>
            </Link>
          ))}
        </div>
      </div>
    </nav>
  );
}
