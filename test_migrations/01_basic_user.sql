-- Insert a basic test user into the database
INSERT INTO "user"
    (email, username, avatar, discord_id, created_at, updated_at)
VALUES
    ('test@test.com', 'test', 'https://cdn.discordapp.com/avatars/1234567890/abcdef.jpg', '1234567890', datetime('now'), datetime('now'));

-- Insert a basic test casino into the database
INSERT INTO "casino"
    (name, url, description, created_at, updated_at)
VALUES
    ('Test', 'testcasino.com', 'Test', datetime('now'), datetime('now'));


--- Insert a basic test transaction into the database
INSERT INTO "transaction"
    (user_id, casino_id, cost, benefit, created_at, updated_at, notes)
VALUES
    (1, 1, 100, 100, datetime('now'), datetime('now'), 'fun');