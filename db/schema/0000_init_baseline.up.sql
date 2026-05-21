CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE TYPE user_status AS ENUM ('anonymous', 'unverified', 'verified',
'affiliate', 'partner', 'banned', 'deleted');
CREATE TYPE stream_status AS ENUM ('planned', 'live', 'ended', 'banned');
CREATE TYPE notification_type AS ENUM ('stream_started', 'new_follower', 'system_alert');
CREATE TYPE moderation_action_type AS ENUM ('ban', 'timeout', 'delete');


CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';
