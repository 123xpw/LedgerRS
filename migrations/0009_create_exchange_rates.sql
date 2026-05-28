CREATE TABLE IF NOT EXISTS exchange_rates (
    id             UUID          PRIMARY KEY DEFAULT gen_random_uuid(),
    base_currency  CHAR(3)       NOT NULL,
    quote_currency CHAR(3)       NOT NULL,
    rate           NUMERIC(18,8) NOT NULL CHECK (rate > 0),
    date           DATE          NOT NULL,
    created_at     TIMESTAMPTZ   NOT NULL DEFAULT NOW(),
    CONSTRAINT uq_exchange_rate UNIQUE (base_currency, quote_currency, date)
);

-- Primary lookup: find rate for a currency pair on or before a given date.
CREATE INDEX idx_exchange_rates_lookup
    ON exchange_rates (base_currency, quote_currency, date DESC);
