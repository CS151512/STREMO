DROP FUNCTION IF EXISTS update_updated_at_column();

DROP TYPE IF EXISTS moderation_action_type;
DROP TYPE IF EXISTS notification_type;
DROP TYPE IF EXISTS stream_status;
DROP TYPE IF EXISTS user_status;

DROP EXTENSION IF EXISTS "pgcrypto";
DROP EXTENSION IF EXISTS "uuid-ossp";
