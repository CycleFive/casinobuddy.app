-- Insert a basic test user into the database
INSERT INTO "user"
    (id, oauth_id, updated_at)
VALUES
    (uuid_nil(), 'test_oauth_id', NOW());

-- Insert a basic test casino into the database
INSERT INTO "casino"
    (id, name, url, description, updated_at)
VALUES
    (uuid_nil(), 'Test', 'testcasino.com', 'Test', NOW());


--- Insert a basic test transaction into the database
INSERT INTO "transaction"
    (user_id, casino_id, cost, benefit, updated_at, notes)
VALUES
    (uuid_nil(), uuid_nil(), 100, 100, NOW(), 'fun');
