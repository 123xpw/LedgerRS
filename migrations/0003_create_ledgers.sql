CREATE TABLE IF NOT EXISTS ledgers (
    id               UUID          PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id          UUID          NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name             VARCHAR(100)  NOT NULL,
    icon             VARCHAR(50)   NOT NULL DEFAULT 'wallet',
    base_currency    CHAR(3)       NOT NULL,
    -- base_currency is immutable after creation; enforced in application layer (FR-2.3).
    initial_balance  NUMERIC(18,6) NOT NULL DEFAULT 0,
    created_at       TIMESTAMPTZ   NOT NULL DEFAULT NOW(),
    updated_at       TIMESTAMPTZ   NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_ledgers_user ON ledgers (user_id);
