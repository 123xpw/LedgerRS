CREATE TABLE IF NOT EXISTS refresh_tokens (
    id         UUID         PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id    UUID         NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token_hash VARCHAR(255) NOT NULL,
    expires_at TIMESTAMPTZ  NOT NULL,
    revoked_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

-- Lookup by hash on every authenticated request.
CREATE INDEX idx_refresh_tokens_hash ON refresh_tokens (token_hash);
-- Used to list/revoke all tokens for a user on logout.
CREATE INDEX idx_refresh_tokens_user ON refresh_tokens (user_id, revoked_at);
