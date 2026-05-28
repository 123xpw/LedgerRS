CREATE TABLE IF NOT EXISTS categories (
    id          UUID         PRIMARY KEY DEFAULT gen_random_uuid(),
    -- NULL → system preset category (FR-4.1); NOT NULL → user-defined for that ledger (FR-4.2).
    ledger_id   UUID         REFERENCES ledgers(id) ON DELETE CASCADE,
    -- NULL → top-level; NOT NULL → second level (FR-4.3, one nesting level only).
    parent_id   UUID         REFERENCES categories(id) ON DELETE CASCADE,
    name        VARCHAR(100) NOT NULL,
    type        VARCHAR(10)  NOT NULL CHECK (type IN ('income', 'expense')),
    is_system   BOOLEAN      NOT NULL DEFAULT FALSE,
    sort_order  INT          NOT NULL DEFAULT 0,
    created_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_categories_ledger ON categories (ledger_id);
CREATE INDEX idx_categories_parent ON categories (parent_id);
