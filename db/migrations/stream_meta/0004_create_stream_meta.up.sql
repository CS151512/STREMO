-- Справочные таблицы - реплицируются на все узлы Citus
CREATE TABLE IF NOT EXISTS categories (
    id VARCHAR(50) PRIMARY KEY, -- e.g., 'cs2', 'just_chatting'
    name VARCHAR(100) NOT NULL,
    is_active BOOLEAN DEFAULT true
);

-- В проде с Citus раскомментировать!!!!!!!!:
-- SELECT create_reference_table('categories');

-- Распределенные таблицы –шардируются по channel_id
CREATE TABLE IF NOT EXISTS streams (
    id UUID DEFAULT uuid_generate_v4(),
    channel_id UUID NOT NULL,
    title VARCHAR(140) NOT NULL,
    category_id VARCHAR(50) REFERENCES categories(id),
    status stream_status DEFAULT 'live',
    viewers_count INTEGER DEFAULT 0,
    started_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    ended_at TIMESTAMP WITH TIME ZONE,
    PRIMARY KEY (channel_id, id) -- В Citus Sharding Key должен быть частью PK!!!!!!
);

-- В проде с Citus раскомментировать!!!!!!!!:
-- SELECT create_distributed_table('streams', 'channel_id');

CREATE INDEX idx_streams_status ON streams(status) WHERE status = 'live';
