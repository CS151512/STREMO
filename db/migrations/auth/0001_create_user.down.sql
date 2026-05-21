DROP INDEX IF EXISTS idx_followers_channel_id;
DROP INDEX IF EXISTS idx_users_email;
DROP INDEX IF EXISTS idx_users_username;

DROP TABLE IF EXISTS followers;

DROP TRIGGER IF EXISTS update_profiles_updated_at ON profiles;
DROP TABLE IF EXISTS profiles;

DROP TRIGGER IF EXISTS update_users_updated_at ON users;
DROP TABLE IF EXISTS users;
