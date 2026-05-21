-- Распределенные таблицы– шардируются по channel_id
CREATE TABLE IF NOT EXISTS moderation_actions (
    id UUID DEFAULT uuid_generate_v4(),
    channel_id UUID NOT NULL,
    target_user_id UUID NOT NULL,
    moderator_id UUID NOT NULL,
    action_type moderation_action_type NOT NULL,
    reason TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (channel_id, id)
);

-- В проде с Citus раскомментировать!!!!!!!!:
-- SELECT create_distributed_table('moderation_actions', 'channel_id');

CREATE INDEX idx_moderation_channel_target ON moderation_actions(channel_id, target_user_id);
