CREATE TABLE IF NOT EXISTS budgets (
    id          UUID          PRIMARY KEY DEFAULT gen_random_uuid(),
    ledger_id   UUID          NOT NULL REFERENCES ledgers(id) ON DELETE CASCADE,
    -- NULL → total budget (FR-5.1); NOT NULL → category budget (FR-5.2).
    category_id UUID          REFERENCES categories(id) ON DELETE CASCADE,
    period      VARCHAR(6)    NOT NULL CHECK (period IN ('week', 'month', 'year')),
    amount      NUMERIC(18,6) NOT NULL CHECK (amount > 0),
    created_at  TIMESTAMPTZ   NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ   NOT NULL DEFAULT NOW(),
    -- One budget per ledger × category × period (null-safe unique constraint).
    CONSTRAINT uq_budget_ledger_cat_period UNIQUE NULLS NOT DISTINCT (ledger_id, category_id, period)
);

CREATE INDEX idx_budgets_ledger ON budgets (ledger_id);
