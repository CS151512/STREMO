-- Сиды для справочных словарей

INSERT INTO categories (id, name, is_active) VALUES
('just_chatting', 'Общение (Just Chatting)', true),
('cs2', 'Counter-Strike 2', true),
('dota2', 'Dota 2', true),
('valorant', 'Valorant', true),
('gta_v', 'Grand Theft Auto V', true),
('league_of_legends', 'League of Legends', true),
('music', 'Музыка (Music)', true),
('asmr', 'ASMR', true)
ON CONFLICT (id) DO NOTHING;

INSERT INTO badges (id, name, icon_url) VALUES
('broadcaster', 'Владелец канала', 'https://cdn.stremo.com/badges/broadcaster.png'),
('moderator', 'Модератор', 'https://cdn.stremo.com/badges/moderator.png'),
('vip', 'VIP', 'https://cdn.stremo.com/badges/vip.png'),
('subscriber', 'Подписчик', 'https://cdn.stremo.com/badges/subscriber.png'),
('verified', 'Подтвержденный аккаунт', 'https://cdn.stremo.com/badges/verified.png'),
('staff', 'Команда STREMO', 'https://cdn.stremo.com/badges/staff.png')
ON CONFLICT (id) DO NOTHING;
