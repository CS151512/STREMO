-- Распределенные таблицы - шардируются по channel_id
CREATE TABLE IF NOT EXISTS vods (
    id UUID DEFAULT uuid_generate_v4(),
    channel_id UUID NOT NULL,
    stream_id UUID NOT NULL,
    title VARCHAR(140) NOT NULL,
    duration_seconds INTEGER NOT NULL,
    views INTEGER DEFAULT 0,
    thumbnail_url VARCHAR(255),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (channel_id, id)
);

-- В проде с Citus раскомментировать!!!!!!!!:
-- SELECT create_distributed_table('vods', 'channel_id');

CREATE INDEX idx_vods_channel_id_created_at ON vods(channel_id, created_at DESC);
