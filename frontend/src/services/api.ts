import axios from 'axios';

const API_URL = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8080';

export const api = axios.create({
  baseURL: `${API_URL}/api/v1`,
  headers: {
    'Content-Type': 'application/json',
  },
});

// Request interceptor for adding auth token
api.interceptors.request.use(
  (config) => {
    const token = localStorage.getItem('token');
    if (token) {
      config.headers.Authorization = `Bearer ${token}`;
    }
    return config;
  },
  (error) => Promise.reject(error)
);

// Response interceptor for handling errors
api.interceptors.response.use(
  (response) => response,
  (error) => {
    if (error.response?.status === 401) {
      // Redirect to login if unauthorized
      localStorage.removeItem('token');
      window.location.href = '/login';
    }
    return Promise.reject(error);
  }
);

// Auth API
export const authAPI = {
  register: async (email: string, username: string, password: string) => {
    const response = await api.post('/auth/register', { email, username, password });
    return response.data;
  },

  login: async (email: string, password: string) => {
    const response = await api.post('/auth/login', { email, password });
    return response.data;
  },
};

// Social API
export const socialAPI = {
  getFeed: async () => {
    const response = await api.get('/social/feed');
    return response.data;
  },

  createPost: async (content: string, visibility: string = 'public') => {
    const response = await api.post('/social/posts', { content, visibility });
    return response.data;
  },
};

// Rides API
export const ridesAPI = {
  requestRide: async (origin: { latitude: number; longitude: number }, destination: { latitude: number; longitude: number }) => {
    const response = await api.post('/rides/request', { origin, destination });
    return response.data;
  },

  getRideStatus: async (rideId: string) => {
    const response = await api.get(`/rides/${rideId}/status`);
    return response.data;
  },
};

// DAO API
export const daoAPI = {
  createProposal: async (title: string, description: string, proposalType: string, votingDurationHours: number) => {
    const response = await api.post('/dao/proposals', {
      title,
      description,
      proposal_type: proposalType,
      voting_duration_hours: votingDurationHours,
    });
    return response.data;
  },

  getProposals: async () => {
    const response = await api.get('/dao/proposals');
    return response.data;
  },
};

// Health check
export const healthCheck = async () => {
  const response = await axios.get(`${API_URL}/health`);
  return response.data;
};
