-- Insert a basic test user into the database
INSERT INTO "user"
    (id, created_at, updated_at)
VALUES
    ('d61b6bba-61ba-4cab-b8b7-74a880968ec6'::UUID, NOW(), NOW()),
    (uuid_nil(), NOW(), NOW());

-- Insert a basic test casino into the database
INSERT INTO "casino"
    (id, name, url, description, created_at, updated_at)
VALUES
    (uuid_nil(), 'Test', 'testcasino.com', 'Test', NOW(), NOW());


--- Insert a basic test transaction into the database
INSERT INTO "transaction"
    (user_id, casino_id, cost, benefit, created_at, updated_at, notes)
VALUES
    ('d61b6bba-61ba-4cab-b8b7-74a880968ec6'::UUID, uuid_nil(), 100, 100, NOW(), NOW(), 'fun');