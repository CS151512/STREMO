CREATE TABLE IF NOT EXISTS wallets (
    user_id UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    bits_balance INTEGER NOT NULL DEFAULT 0 CHECK (bits_balance >= 0),
    fiat_balance NUMERIC(12,2) NOT NULL DEFAULT 0.00 CHECK (fiat_balance >= 0.00),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_wallets_updated_at
    BEFORE UPDATE ON wallets
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TABLE IF NOT EXISTS outbox_events (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    aggregate_type VARCHAR(100) NOT NULL, -- e.g., 'billing'
    aggregate_id VARCHAR(100) NOT NULL,   -- e.g., 'user_id'
    event_type VARCHAR(100) NOT NULL,     -- e.g., 'payment.received'
    payload JSONB NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Индекс для парсинга (CDC)
CREATE INDEX idx_outbox_events_created_at ON outbox_events(created_at);
