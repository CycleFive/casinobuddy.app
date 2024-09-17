-- Insert a basic test user into the database
INSERT INTO "user"
    (email, username, avatar, discord_id, created_at, updated_at)
VALUES
    ('test@test.com', 'test', 'https://cdn.discordapp.com/avatars/1234567890/abcdef.jpg', '1234567890', date('now'), date('now'));