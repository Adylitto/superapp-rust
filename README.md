# SuperApp - Ultra-Secure All-in-One Platform

A WeChat-inspired super app built with Rust, featuring social networking, real-time messaging, ride-sharing, payments, DAO governance, and AI-powered features.

## 🏗️ Architecture

Built using Clean Architecture and Domain-Driven Design principles:

- **app-core**: Domain entities, use cases, and business logic
- **app-api**: REST/GraphQL API gateway with WebSocket support
- **app-dao**: Blockchain smart contracts (Solana/Anchor)
- **app-ai**: AI/ML integrations for recommendations and automation
- **app-social**: Social networking features
- **app-mobility**: Ride-sharing and geolocation services
- **app-payments**: Payment processing (fiat + crypto)
- **app-messaging**: Real-time encrypted messaging
- **app-auth**: Authentication and authorization
- **app-storage**: Database and caching adapters

## 🚀 Key Features

### Security
- ✅ Zero-trust architecture with JWT/OAuth
- ✅ End-to-end encryption (ring/rustls)
- ✅ Input validation and sanitization
- ✅ Type-safe design with Rust ownership model
- ✅ Regular vulnerability scanning (cargo-audit)

### Blockchain & DAO
- ✅ Solana-based smart contracts (Anchor framework)
- ✅ Token-based governance with quadratic voting
- ✅ Automated proposal execution
- ✅ Token rewards for user engagement

### AI Features
- ✅ Personalized recommendations (social connections, content)
- ✅ Predictive ride matching and routing optimization
- ✅ Content moderation and safety
- ✅ Privacy-preserving federated learning
- ✅ AI agents for task automation

### Core Modules
- 📱 **Social Networking**: Feeds, posts, friends, AI-powered discovery
- 💬 **Messaging**: Real-time E2E encrypted chat
- 🚗 **Ride-Sharing**: Geolocation matching, AI route optimization
- 💰 **Payments**: Fiat/crypto wallet with cross-border support
- 🎯 **Mini-Apps**: Third-party ecosystem with DAO governance
- 🌍 **Enhancements**: Multilingual AI, sustainability tracking, AR previews

## 📋 Prerequisites

### Required
- Rust 1.75.0+ (`rustup install stable`)
- PostgreSQL 15+ (or Supabase account)

### Optional but Recommended
- Redis 7+ (for caching and rate limiting)
- Docker & Docker Compose (easiest way to run dependencies)

### For Blockchain Features
- Solana CLI 1.17+ (`sh -c "$(curl -sSfL https://release.solana.com/stable/install)"`)
- Anchor 0.29+ (`cargo install --git https://github.com/coral-xyz/anchor anchor-cli --locked`)

**See [docs/INSTALL_DEPENDENCIES.md](./docs/INSTALL_DEPENDENCIES.md) for detailed installation instructions.**

## 🛠️ Quick Start

### Option 1: Using Supabase (Recommended for Easy Setup)

1. **Create Supabase Account**:
   - Sign up at [https://supabase.com](https://supabase.com)
   - Create a new project
   - Get your project connection details

2. **Configure Environment**:
   ```bash
   cp .env.example .env
   # Edit .env with your Supabase details
   ```

3. **Set Up Database Schema**:
   - Go to Supabase SQL Editor
   - Copy contents of `crates/app-storage/migrations/001_initial_schema.sql`
   - Execute to create tables

4. **Run the Application**:
   ```bash
   cargo run -p app-api
   ```

### Option 2: Local Development with Docker

```bash
# Use Docker for dependencies (recommended)
docker-compose -f docker-compose.dev.yml up -d
cargo run -p app-api
```

### Option 3: Full Docker
```bash
docker-compose up -d
```

### Option 4: All Local (requires PostgreSQL & Redis)
```bash
./scripts/setup.sh
cargo run -p app-api
```

**📖 Detailed guide:** See [docs/QUICK_START.md](./docs/QUICK_START.md)
**📘 Supabase Integration Guide:** See [docs/SUPABASE_INTEGRATION.md](./docs/SUPABASE_INTEGRATION.md)

## 🔧 Configuration

Edit `config/default.toml`:

```toml
[server]
host = "0.0.0.0"
port = 8080

[database]
url = "postgresql://user:pass@localhost:5432/superapp"
max_connections = 100

[redis]
url = "redis://localhost:6379"

[blockchain]
network = "devnet"
program_id = "your-program-id"

[ai]
model_path = "./models/recommendation.onnx"
inference_threads = 4

[security]
jwt_secret = "your-secret-key"
token_expiry = 3600
```

### 2. Database Setup

```bash
# Start PostgreSQL and Redis with Docker
docker-compose up -d postgres redis

# Run migrations
cd crates/app-storage
sqlx migrate run
```

### 3. Build Smart Contracts

```bash
# Switch to Solana devnet
solana config set --url devnet

# Build and deploy DAO contracts
cd crates/app-dao
anchor build
anchor deploy

# Update program IDs in config
```

### 4. Run Development Server

```bash
# Start API server
cargo run -p app-api

# Server runs on http://localhost:8080
```

### 5. Run Tests

```bash
# Unit tests
cargo test --workspace

# Integration tests
cargo test --workspace --test '*'

# Security audit
cargo audit
```

## 🔧 Configuration

Edit `config/default.toml`:

```toml
[server]
host = "0.0.0.0"
port = 8080

[database]
url = "postgresql://user:pass@localhost:5432/superapp"
max_connections = 100

[redis]
url = "redis://localhost:6379"

[blockchain]
network = "devnet"
program_id = "your-program-id"

[ai]
model_path = "./models/recommendation.onnx"
inference_threads = 4

[security]
jwt_secret = "your-secret-key"
token_expiry = 3600
```

## 🏛️ DAO Token Economics

### Token Rewards
- **Social Actions**: 5 tokens per post with >10 likes
- **Ride Completion**: 10 tokens per ride
- **Referrals**: 20 tokens per successful referral
- **Mini-App Usage**: 2 tokens per session

### Governance
- **Voting Power**: 1 token = 1 vote (quadratic voting option)
- **Proposal Threshold**: 1000 tokens to create
- **Quorum**: 10% of circulating supply
- **Execution Delay**: 48 hours

## 🤖 AI Integration

### Recommendation Engine
```rust
// Example: Get AI-powered friend suggestions
let suggestions = ai_service
    .recommend_connections(user_id, limit)
    .await?;
```

### Ride Optimization
```rust
// Example: AI-optimized route calculation
let route = ai_service
    .optimize_route(origin, destination, constraints)
    .await?;
```

## 📊 API Endpoints

### Authentication
- `POST /api/v1/auth/register` - User registration
- `POST /api/v1/auth/login` - Login with JWT
- `POST /api/v1/auth/refresh` - Refresh token

### Social
- `GET /api/v1/social/feed` - Get personalized feed
- `POST /api/v1/social/posts` - Create post
- `GET /api/v1/social/recommendations` - AI-powered suggestions

### Mobility
- `POST /api/v1/rides/request` - Request ride
- `GET /api/v1/rides/{id}/status` - Track ride
- `POST /api/v1/rides/{id}/complete` - Complete & earn tokens

### DAO
- `POST /api/v1/dao/proposals` - Create proposal
- `POST /api/v1/dao/proposals/{id}/vote` - Vote on proposal
- `GET /api/v1/dao/governance` - View governance stats

### Messaging
- `WS /api/v1/ws/chat` - WebSocket chat connection

## 🚀 Deployment

### Docker Compose (Development)
```bash
docker-compose up -d
```

### Kubernetes (Production)
```bash
# Build images
docker build -t superapp-api:latest .

# Deploy to k8s
kubectl apply -f k8s/

# Scale horizontally
kubectl scale deployment superapp-api --replicas=10
```

## 🔐 Security Best Practices

1. **Input Validation**: All inputs validated with custom validators
2. **Rate Limiting**: Tower middleware with Redis backend
3. **CORS**: Strict origin policies
4. **Encryption**: E2E for messages, TLS for transport
5. **Audit Logs**: All sensitive operations logged
6. **Dependency Scanning**: Automated `cargo-audit` in CI/CD

## 🧪 Testing Strategy

- **Unit Tests**: 80%+ coverage target
- **Integration Tests**: API endpoint testing
- **Smart Contract Tests**: Anchor test suite
- **Load Tests**: k6 for performance validation
- **Security Tests**: OWASP ZAP scanning

## 📈 Performance Targets

- **Throughput**: 10,000+ req/s per node
- **Latency**: <50ms p99 for API calls
- **Scalability**: Horizontal scaling to millions of users
- **Availability**: 99.9% uptime SLA

## 🔮 Roadmap

### Phase 1 (Current)
- ✅ Core architecture setup
- ✅ Basic DAO functionality
- ✅ Authentication system
- 🔄 Messaging service
- 🔄 Token rewards system

### Phase 2 (Q2 2025)
- [ ] AI recommendation engine
- [ ] Ride-sharing MVP
- [ ] Payment integration
- [ ] Mini-apps platform

### Phase 3 (Q3 2025)
- [ ] Advanced AI agents
- [ ] Cross-border payments
- [ ] AR features
- [ ] Sustainability tracking

### Phase 4 (Q4 2025)
- [ ] Global expansion
- [ ] Advanced DAO features
- [ ] Enterprise partnerships

## 🤝 Contributing

1. Fork the repository
2. Create feature branch (`git checkout -b feature/amazing-feature`)
3. Commit changes (`git commit -m 'Add amazing feature'`)
4. Push to branch (`git push origin feature/amazing-feature`)
5. Open Pull Request

## 📄 License

MIT OR Apache-2.0

## 📞 Support

- Documentation: [docs/](./docs)
- Issues: GitHub Issues
- Discord: [Community Server]
- Email: support@superapp.io

## 🙏 Acknowledgments

- Rust community
- Solana/Anchor teams
- Open-source contributors

---

**Built with ❤️ using Rust for maximum security and performance**
