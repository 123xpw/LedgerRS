CREATE TABLE IF NOT EXISTS transactions (
    id               UUID          PRIMARY KEY DEFAULT gen_random_uuid(),
    ledger_id        UUID          NOT NULL REFERENCES ledgers(id) ON DELETE CASCADE,
    type             VARCHAR(10)   NOT NULL CHECK (type IN ('income', 'expense', 'transfer')),
    amount           NUMERIC(18,6) NOT NULL CHECK (amount > 0),
    currency         CHAR(3)       NOT NULL,
    -- Denormalized: amount converted to ledger base_currency at occurred_at.
    -- Stored to avoid per-query exchange-rate joins in aggregations.
    base_amount      NUMERIC(18,6) NOT NULL,
    -- Rate used for the above conversion (= 1 when same currency).
    applied_rate     NUMERIC(18,8) NOT NULL DEFAULT 1,
    category_id      UUID          REFERENCES categories(id) ON DELETE SET NULL,
    note             TEXT,
    occurred_at      TIMESTAMPTZ   NOT NULL,
    -- User override for the exchange rate (FR-6.5).
    custom_rate      NUMERIC(18,8),
    -- Both sides of a transfer share the same transfer_pair_id (FR-3.2).
    transfer_pair_id UUID,
    created_at       TIMESTAMPTZ   NOT NULL DEFAULT NOW(),
    updated_at       TIMESTAMPTZ   NOT NULL DEFAULT NOW()
);

-- Primary listing query: ledger + date descending.
CREATE INDEX idx_transactions_ledger_date ON transactions (ledger_id, occurred_at DESC);
-- Budget aggregation query.
CREATE INDEX idx_transactions_category    ON transactions (category_id);
-- Transfer pair lookup.
CREATE INDEX idx_transactions_transfer    ON transactions (transfer_pair_id)
    WHERE transfer_pair_id IS NOT NULL;
-- Full-text note search (GIN index on tsvector for better performance).
CREATE INDEX idx_transactions_note_gin    ON transactions
    USING GIN (to_tsvector('simple', COALESCE(note, '')));
