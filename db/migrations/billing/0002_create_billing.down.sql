DROP INDEX IF EXISTS idx_outbox_events_created_at;
DROP TABLE IF EXISTS outbox_events;

DROP TRIGGER IF EXISTS update_wallets_updated_at ON wallets;
DROP TABLE IF EXISTS wallets;
