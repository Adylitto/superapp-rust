'use client';

import { useState } from 'react';
import Link from 'next/link';

export default function SocialPage() {
  const [posts, setPosts] = useState([
    {
      id: 1,
      author: 'John Doe',
      content: 'Just joined SuperApp! Excited to be part of this amazing community.',
      timestamp: '2 hours ago',
      likes: 12,
      comments: 3,
    },
    {
      id: 2,
      author: 'Jane Smith',
      content: 'The new DAO governance features are incredible! Really empowering to have a say in the direction of the platform.',
      timestamp: '5 hours ago',
      likes: 24,
      comments: 7,
    },
    {
      id: 3,
      author: 'Alex Johnson',
      content: 'Completed my first ride with SuperApp today. The AI-optimized routing saved me 15 minutes!',
      timestamp: '1 day ago',
      likes: 18,
      comments: 2,
    }
  ]);

  const [newPost, setNewPost] = useState('');

  const handlePostSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (newPost.trim()) {
      const post = {
        id: posts.length + 1,
        author: 'You',
        content: newPost,
        timestamp: 'Just now',
        likes: 0,
        comments: 0,
      };
      setPosts([post, ...posts]);
      setNewPost('');
    }
  };

  return (
    <div className="min-h-screen bg-gray-50">
      {/* Header */}
      <header className="bg-white shadow-sm">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex justify-between h-16 items-center">
            <Link href="/" className="flex items-center">
              <div className="h-10 w-10 rounded-full bg-blue-500 flex items-center justify-center">
                <span className="text-white font-bold">SA</span>
              </div>
              <span className="ml-3 text-xl font-bold text-gray-900">SuperApp</span>
            </Link>
            <nav className="flex space-x-8">
              <Link href="/social" className="text-blue-600 font-medium">Social</Link>
              <Link href="/messages" className="text-gray-500 hover:text-gray-700 font-medium">Messages</Link>
              <Link href="/rides" className="text-gray-500 hover:text-gray-700 font-medium">Rides</Link>
              <Link href="/dao" className="text-gray-500 hover:text-gray-700 font-medium">DAO</Link>
              <Link href="/profile" className="text-gray-500 hover:text-gray-700 font-medium">Profile</Link>
            </nav>
          </div>
        </div>
      </header>

      <main className="max-w-3xl mx-auto px-4 py-8">
        {/* Create Post */}
        <div className="bg-white rounded-lg shadow p-6 mb-6">
          <form onSubmit={handlePostSubmit}>
            <div className="flex">
              <div className="flex-shrink-0 mr-3">
                <div className="h-10 w-10 rounded-full bg-gray-300 flex items-center justify-center">
                  <span className="text-gray-600 font-bold">Y</span>
                </div>
              </div>
              <div className="flex-1">
                <textarea
                  value={newPost}
                  onChange={(e) => setNewPost(e.target.value)}
                  placeholder="What's on your mind?"
                  className="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 resize-none"
                  rows={3}
                />
                <div className="mt-4 flex justify-end">
                  <button
                    type="submit"
                    className="px-6 py-2 bg-blue-600 text-white font-medium rounded-lg hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
                  >
                    Post
                  </button>
                </div>
              </div>
            </div>
          </form>
        </div>

        {/* Posts Feed */}
        <div className="space-y-6">
          {posts.map((post) => (
            <div key={post.id} className="bg-white rounded-lg shadow">
              <div className="p-6">
                <div className="flex items-center mb-4">
                  <div className="h-10 w-10 rounded-full bg-gray-300 flex items-center justify-center">
                    <span className="text-gray-600 font-bold">{post.author.charAt(0)}</span>
                  </div>
                  <div className="ml-3">
                    <h3 className="font-bold text-gray-900">{post.author}</h3>
                    <p className="text-sm text-gray-500">{post.timestamp}</p>
                  </div>
                </div>
                <p className="text-gray-800 mb-4">{post.content}</p>
                <div className="flex space-x-6 text-gray-500">
                  <button className="flex items-center space-x-1 hover:text-blue-600">
                    <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z"></path>
                    </svg>
                    <span>{post.likes}</span>
                  </button>
                  <button className="flex items-center space-x-1 hover:text-blue-600">
                    <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z"></path>
                    </svg>
                    <span>{post.comments}</span>
                  </button>
                </div>
              </div>
            </div>
          ))}
        </div>
      </main>
    </div>
  );
}