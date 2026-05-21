-- Справочные таблицы
CREATE TABLE IF NOT EXISTS badges (
    id VARCHAR(50) PRIMARY KEY, -- e.g., 'vip', 'subscriber', 'moderator'
    name VARCHAR(100) NOT NULL,
    icon_url VARCHAR(255) NOT NULL
);
-- В проде с Citus раскомментировать!!!!!!!!:
-- SELECT create_reference_table('badges');

-- Распределенные таблицы – шардируются по channel_id
CREATE TABLE IF NOT EXISTS chat_messages (
    id UUID DEFAULT uuid_generate_v4(),
    channel_id UUID NOT NULL,
    user_id UUID NOT NULL,
    text TEXT NOT NULL,
    badges TEXT[] DEFAULT '{}',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (channel_id, id)
);
-- В проде с Citus раскомментировать!!!!!!!!:
-- SELECT create_distributed_table('chat_messages', 'channel_id');

CREATE INDEX idx_chat_messages_created_at ON chat_messages(channel_id, created_at DESC);
