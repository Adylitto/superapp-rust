-- Create users table
CREATE TABLE users (
    id UUID PRIMARY KEY,
    email VARCHAR(255) UNIQUE NOT NULL,
    username VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    profile_full_name VARCHAR(255) NOT NULL DEFAULT '',
    profile_avatar_url TEXT,
    profile_bio TEXT,
    profile_location TEXT,
    profile_phone VARCHAR(50),
    profile_preferences JSONB NOT NULL DEFAULT '{}',
    token_balance BIGINT NOT NULL DEFAULT 0,
    reputation_score DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    is_verified BOOLEAN NOT NULL DEFAULT FALSE,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Create posts table
CREATE TABLE posts (
    id UUID PRIMARY KEY,
    author_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    media_urls JSONB NOT NULL DEFAULT '[]',
    likes_count INTEGER NOT NULL DEFAULT 0,
    comments_count INTEGER NOT NULL DEFAULT 0,
    shares_count INTEGER NOT NULL DEFAULT 0,
    visibility VARCHAR(50) NOT NULL DEFAULT 'Public',
    is_ai_moderated BOOLEAN NOT NULL DEFAULT FALSE,
    moderation_score DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    tokens_earned BIGINT NOT NULL DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Create messages table
CREATE TABLE messages (
    id UUID PRIMARY KEY,
    from_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    to_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    is_encrypted BOOLEAN NOT NULL DEFAULT FALSE,
    is_read BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Create rides table
CREATE TABLE rides (
    id UUID PRIMARY KEY,
    rider_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    driver_id UUID REFERENCES users(id) ON DELETE SET NULL,
    origin_latitude DOUBLE PRECISION NOT NULL,
    origin_longitude DOUBLE PRECISION NOT NULL,
    origin_address TEXT,
    destination_latitude DOUBLE PRECISION NOT NULL,
    destination_longitude DOUBLE PRECISION NOT NULL,
    destination_address TEXT,
    distance DOUBLE PRECISION, -- in meters
    duration INTEGER, -- in seconds
    estimated_cost DECIMAL(10,2),
    actual_cost DECIMAL(10,2),
    status VARCHAR(50) NOT NULL DEFAULT 'requested',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    started_at TIMESTAMP WITH TIME ZONE,
    completed_at TIMESTAMP WITH TIME ZONE
);

-- Create proposals table
CREATE TABLE proposals (
    id UUID PRIMARY KEY,
    proposer_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title VARCHAR(500) NOT NULL,
    description TEXT,
    proposal_type VARCHAR(100) NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'Draft',
    execution_data TEXT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    voting_starts_at TIMESTAMP WITH TIME ZONE,
    voting_ends_at TIMESTAMP WITH TIME ZONE,
    executed_at TIMESTAMP WITH TIME ZONE
);

-- Create proposal_votes table
CREATE TABLE proposal_votes (
    id UUID PRIMARY KEY,
    proposal_id UUID NOT NULL REFERENCES proposals(id) ON DELETE CASCADE,
    voter_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    vote_type VARCHAR(20) NOT NULL, -- 'For', 'Against', 'Abstain'
    voting_power BIGINT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    UNIQUE(proposal_id, voter_id) -- Prevent duplicate votes
);

-- Create token_transactions table
CREATE TABLE token_transactions (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    from_user_id UUID REFERENCES users(id) ON DELETE SET NULL, -- NULL for rewards
    to_user_id UUID REFERENCES users(id) ON DELETE SET NULL,   -- NULL for burns
    amount BIGINT NOT NULL,
    reason VARCHAR(100) NOT NULL,
    transaction_hash VARCHAR(255),
    status VARCHAR(50) NOT NULL DEFAULT 'pending',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Create indexes for better performance
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_username ON users(username);
CREATE INDEX idx_posts_author_id ON posts(author_id);
CREATE INDEX idx_posts_created_at ON posts(created_at);
CREATE INDEX idx_messages_from_id ON messages(from_id);
CREATE INDEX idx_messages_to_id ON messages(to_id);
CREATE INDEX idx_messages_created_at ON messages(created_at);
CREATE INDEX idx_rides_rider_id ON rides(rider_id);
CREATE INDEX idx_rides_driver_id ON rides(driver_id);
CREATE INDEX idx_proposals_proposer_id ON proposals(proposer_id);
CREATE INDEX idx_proposals_status ON proposals(status);
CREATE INDEX idx_proposals_voting_ends_at ON proposals(voting_ends_at);
CREATE INDEX idx_token_transactions_user_id ON token_transactions(user_id);
CREATE INDEX idx_token_transactions_created_at ON token_transactions(created_at);