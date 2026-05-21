DO $$
DECLARE
    streamer1_id UUID := '11111111-1111-1111-1111-111111111111';
    streamer2_id UUID := '22222222-2222-2222-2222-222222222222';
    viewer1_id UUID := '33333333-3333-3333-3333-333333333333';
    viewer2_id UUID := '44444444-4444-4444-4444-444444444444';
    stream1_id UUID := '55555555-5555-5555-5555-555555555555';
BEGIN

    INSERT INTO users (id, username, email, password_hash, status) VALUES
    (streamer1_id, 'streamer_pro', 'pro@stremo.com',
    '$argon2id$v=19$m=65536,t=3,p=4$Jc...', 'partner'),
    (streamer2_id, 'chill_guy', 'chill@stremo.com', '
    $argon2id$v=19$m=65536,t=3,p=4$Jc...', 'affiliate'),
    (viewer1_id, 'angry_viewer', 'viewer1@stremo.com',
    '$argon2id$v=19$m=65536,t=3,p=4$Jc...', 'verified'),
    (viewer2_id, 'rich_kid', 'rich@stremo.com',
    '$argon2id$v=19$m=65536,t=3,p=4$Jc...', 'verified')
    ON CONFLICT (id) DO NOTHING;

    INSERT INTO profiles (user_id, bio, avatar_url, is_verified, followers_count) VALUES
    (streamer1_id, 'Professional CS2 player! Road to Global!',
    'https://cdn.stremo.com/avatars/1.png', true, 15400),
    (streamer2_id, 'Just relaxing and chatting.',
    'https://cdn.stremo.com/avatars/2.png', false, 320),
    (viewer1_id, 'I love watching CS2', null, false, 0),
    (viewer2_id, 'Donate machine', 'https://cdn.stremo.com/avatars/4.png', false, 12)
    ON CONFLICT (user_id) DO NOTHING;

    INSERT INTO wallets (user_id, bits_balance, fiat_balance) VALUES
    (streamer1_id, 45000, 1500.50),
    (streamer2_id, 1200, 25.00),
    (viewer1_id, 0, 0.00),
    (viewer2_id, 50000, 0.00)
    ON CONFLICT (user_id) DO NOTHING;

    INSERT INTO followers (follower_id, channel_id) VALUES
    (viewer1_id, streamer1_id),
    (viewer2_id, streamer1_id),
    (viewer2_id, streamer2_id)
    ON CONFLICT DO NOTHING;

    INSERT INTO streams (id, channel_id, title, category_id, status, viewers_count) VALUES
    (stream1_id, streamer1_id, 'Road to Global Elite | Drop Crate', 'cs2', 'live', 15405),
    (uuid_generate_v4(), streamer2_id, 'Night vibes & Music', 'just_chatting', 'live', 120)
    ON CONFLICT DO NOTHING;

    INSERT INTO chat_messages (channel_id, user_id, text, badges) VALUES
    (streamer1_id, viewer1_id, 'GG WP!', '{"subscriber"}'),
    (streamer1_id, viewer2_id, 'Hello streamer, nice aim!', '{"vip"}'),
    (streamer1_id, streamer1_id, 'Thanks guys!', '{"broadcaster", "verified"}')
    ON CONFLICT DO NOTHING;

    INSERT INTO vods (channel_id, stream_id, title, duration_seconds, views) VALUES
    (streamer1_id, stream1_id, 'Past Stream VOD - CS2 Final', 7200, 1500)
    ON CONFLICT DO NOTHING;

    INSERT INTO moderation_actions (channel_id, target_user_id,
    moderator_id, action_type, reason) VALUES
    (streamer1_id, viewer1_id, streamer1_id, 'timeout', 'Spamming in chat')
    ON CONFLICT DO NOTHING;

    INSERT INTO notifications (user_id, type, title, body, action_url) VALUES
    (viewer1_id, 'stream_started', 'streamer_pro начал трансляцию!',
    'Заходи смотреть Road to Global Elite', 'https://stremo.com/streamer_pro')
    ON CONFLICT DO NOTHING;

END $$;
