# SuperApp-Rust Project Context

## 🏗️ Project Overview

This is a sophisticated **WeChat-inspired super app** built with Rust, featuring social networking, real-time messaging, ride-sharing, payments, DAO governance, and AI-powered features. The architecture follows Clean Architecture and Domain-Driven Design principles with a modular crate structure.

**Core Architecture:**
- **app-core**: Domain entities, use cases, and business logic
- **app-api**: REST/GraphQL API gateway with WebSocket support
- **app-ai**: AI/ML integrations for recommendations and automation
- **app-social**: Social networking features
- **app-mobility**: Ride-sharing and geolocation services
- **app-payments**: Payment processing (fiat + crypto)
- **app-messaging**: Real-time encrypted messaging
- **app-auth**: Authentication and authorization
- **app-storage**: Database and caching adapters
- **app-dao**: Blockchain smart contracts (Solana/Anchor)

## 🛠️ Technologies & Stack

**Backend Framework:** 
- Rust 1.75+ with async ecosystem (Tokio, Axum)
- HTTP framework: Axum with Tower middleware
- Database: PostgreSQL with SQLx
- Caching: Redis
- Security: Ring, Rustls, JWT, Argon2

**Blockchain:**
- Solana/Anchor framework for smart contracts
- Token-based governance

**AI/ML:**
- ONNX runtime with Tract for model inference
- ndarray for numerical computations

**Deployment:**
- Docker/Docker Compose
- Fly.io for cloud deployment
- Kubernetes-ready configuration

## 🚀 Building and Running

### Prerequisites
```bash
# Required
Rust 1.75.0+
PostgreSQL 15+
Redis 7+ (recommended)

# For blockchain features (optional)
Solana CLI 1.17+
Anchor 0.29+
```

### Development Setup
```bash
# Quick start with Docker for dependencies
cd superapp-rust
docker-compose -f docker-compose.dev.yml up -d
cargo run -p app-api

# Or complete Docker setup
docker-compose up -d

# Or local setup
./scripts/setup.sh
cargo run -p app-api
```

### Database Setup
```bash
# Start PostgreSQL and Redis with Docker
docker-compose up -d postgres redis

# Run migrations
cd crates/app-storage
sqlx migrate run
```

### Testing
```bash
# Unit tests
cargo test --workspace

# Integration tests
cargo test --workspace --test '*'

# Security audit
cargo audit
```

### Smart Contract Development
```bash
# Switch to Solana devnet
solana config set --url devnet

# Build and deploy DAO contracts
cd crates/app-dao
anchor build
anchor deploy

# Update program IDs in config
```

## 🏛️ Configuration

The application uses a TOML-based configuration system with `config/default.toml` as the primary configuration file. Environment variables can override settings.

Key configuration sections:
- Server: host, port, worker threads
- Database: connection pool settings
- Redis: connection pool and timeout
- Security: token expiry, password requirements
- Blockchain: network settings
- AI: inference threads, batch size
- Rate limiting: requests per minute
- CORS and logging settings

## 🔒 Security Features

- **Zero-trust architecture** with JWT/OAuth
- **End-to-end encryption** using ring/rustls
- **Input validation and sanitization**
- **Type-safe design** with Rust ownership model
- **Rate limiting** with Tower middleware and Redis
- **CORS protection** with strict origin policies
- **Regular vulnerability scanning** with cargo-audit

## 🤖 AI Integration

The platform includes AI-powered features:
- Personalized recommendations for social connections and content
- Predictive ride matching and routing optimization
- Content moderation and safety features
- Privacy-preserving federated learning
- AI agents for task automation

## 📊 API Endpoints Structure

The API follows REST/GraphQL patterns with WebSocket support:

- `/api/v1/auth/*` - Authentication
- `/api/v1/social/*` - Social networking features
- `/api/v1/rides/*` - Mobility/Ride-sharing
- `/api/v1/dao/*` - DAO governance
- `/api/v1/ws/chat` - WebSocket chat

## 🚀 Deployment

### Local/Docker
```bash
docker-compose up -d
```

### Fly.io
The project is configured for Fly.io deployment with `fly.toml`:
```bash
fly launch
fly deploy
```

### Production Kubernetes
Docker image builds are configured for containerized deployment with horizontal scaling capabilities.

## 🧪 Testing Strategy

- **Unit Tests**: 80%+ coverage target
- **Integration Tests**: API endpoint testing
- **Smart Contract Tests**: Anchor test suite
- **Load Tests**: k6 for performance validation
- **Security Tests**: OWASP ZAP scanning

## 📈 Performance Targets

- Throughput: 10,000+ req/s per node
- Latency: <50ms p99 for API calls
- Scalability: Horizontal scaling to millions of users
- Availability: 99.9% uptime SLA

## 📁 Directory Structure

```
superapp-rust/
├── Cargo.toml                 # Workspace configuration
├── Dockerfile                 # Multi-stage Docker build
├── docker-compose.yml         # Docker services configuration
├── config/                    # Application configuration
│   └── default.toml
├── crates/                    # Multi-crate workspace
│   ├── app-api/              # Main API server
│   ├── app-core/             # Core business logic
│   ├── app-ai/               # AI/ML components
│   ├── app-social/           # Social networking features
│   ├── app-mobility/         # Ride-sharing services
│   ├── app-payments/         # Payment processing
│   ├── app-messaging/        # Real-time messaging
│   ├── app-auth/             # Authentication
│   └── app-storage/          # Database adapters
├── scripts/                   # Setup and deployment scripts
└── docs/                      # Documentation
```

## 🤝 Development Conventions

- **Code Style**: Follow Rust community best practices and idioms
- **Testing**: Comprehensive test coverage with unit, integration, and load tests
- **Security**: Input validation, type safety, and security scanning
- **Documentation**: Inline code documentation and external documentation in docs/
- **Version Control**: Git feature branches with conventional commits