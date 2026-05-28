CREATE TABLE IF NOT EXISTS tags (
    id         UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    ledger_id  UUID        NOT NULL REFERENCES ledgers(id) ON DELETE CASCADE,
    name       VARCHAR(50) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT uq_tags_ledger_name UNIQUE (ledger_id, name)
);

CREATE INDEX idx_tags_ledger ON tags (ledger_id);
