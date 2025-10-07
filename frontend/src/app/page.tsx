import Link from 'next/link';

const features = [
  {
    title: 'Social Network',
    description: 'Connect with friends, share posts, and discover new connections',
    href: '/social',
  },
  {
    title: 'Real-Time Chat',
    description: 'End-to-end encrypted messaging with real-time updates',
    href: '/messages',
  },
  {
    title: 'Ride Sharing',
    description: 'Request rides with AI-optimized routing and driver matching',
    href: '/rides',
  },
  {
    title: 'Payments',
    description: 'Seamless fiat and crypto payments with built-in wallet',
    href: '/wallet',
  },
  {
    title: 'DAO Governance',
    description: 'Participate in decentralized decision-making with token voting',
    href: '/dao',
  },
  {
    title: 'Mini Apps',
    description: 'Access third-party services and earn tokens for engagement',
    href: '/apps',
  },
];

export default function Home() {
  return (
    <div className="min-h-screen bg-gradient-to-br from-blue-500 to-purple-600 text-white">
      {/* Hero Section */}
      <section className="py-20 px-4">
        <div className="container mx-auto max-w-6xl">
          <div className="text-center">
            <h1 className="text-4xl md:text-6xl font-bold mb-6">
              Welcome to SuperApp
            </h1>
            <p className="text-xl md:text-2xl mb-8 text-white/90">
              Your all-in-one platform for social, mobility, payments, and governance
            </p>
            <div className="flex gap-4 justify-center flex-wrap">
              <Link href="/register">
                <button className="bg-white text-blue-600 px-8 py-4 rounded-full font-bold text-lg shadow-lg hover:shadow-xl transition-all">
                  Get Started
                </button>
              </Link>
              <Link href="/login">
                <button className="bg-white/10 backdrop-blur-sm text-white px-8 py-4 rounded-full font-bold text-lg border-2 border-white/30 hover:bg-white/20 transition-all">
                  Sign In
                </button>
              </Link>
            </div>
          </div>
        </div>
      </section>

      {/* Features Grid */}
      <section className="py-20 px-4 bg-white text-gray-800">
        <div className="container mx-auto max-w-6xl">
          <div className="text-center mb-16">
            <h2 className="text-3xl md:text-4xl font-bold mb-4">
              Everything You Need
            </h2>
            <p className="text-lg text-gray-600">
              Powered by Rust, secured by blockchain, enhanced by AI
            </p>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
            {features.map((feature, index) => (
              <div key={feature.title} className="group">
                <Link href={feature.href}>
                  <div className="bg-white rounded-xl shadow-md border border-gray-200 p-6 hover:shadow-lg transition-shadow h-full cursor-pointer">
                    <div className="w-12 h-12 rounded-xl bg-gradient-to-br from-blue-500 to-cyan-500 flex items-center justify-center mb-4 mx-auto">
                      <span className="text-white text-xl font-bold">{index + 1}</span>
                    </div>
                    <h3 className="text-xl font-bold mb-2 text-center group-hover:text-blue-600 transition-colors">
                      {feature.title}
                    </h3>
                    <p className="text-gray-600 text-center">
                      {feature.description}
                    </p>
                  </div>
                </Link>
              </div>
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
              <div key={stat.label}>
                <div className="text-3xl md:text-4xl font-bold text-blue-600 mb-2">
                  {stat.value}
                </div>
                <div className="text-gray-600 font-medium">
                  {stat.label}
                </div>
              </div>
            ))}
          </div>
        </div>
      </section>

      {/* CTA Section */}
      <section className="py-20 px-4">
        <div className="container mx-auto max-w-4xl">
          <div className="bg-gradient-to-br from-blue-500 to-purple-600 text-white rounded-3xl p-12 text-center shadow-2xl">
            <h2 className="text-3xl md:text-4xl font-bold mb-4">
              Ready to Join?
            </h2>
            <p className="text-xl mb-8 text-white/90">
              Start earning tokens for your engagement today
            </p>
            <Link href="/register">
              <button className="bg-white text-blue-600 px-10 py-4 rounded-full font-bold text-lg shadow-lg hover:shadow-xl transition-all">
                Create Free Account
              </button>
            </Link>
          </div>
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