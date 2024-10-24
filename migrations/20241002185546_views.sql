---
--- Views for various summaries of the data and pages we want to display in the app.
---

-- Create a view to get the spend and benefit for each user
CREATE OR REPLACE VIEW user_spend_benefit AS SELECT
    user_id,
    SUM(cost) AS spend,
    SUM(benefit) AS benefit,
    (SUM(benefit) - SUM(cost)) / SUM(cost) AS average_bonus,
    COUNT(*) AS transactions,
    MAX(created_at) AS last_transaction,
    MIN(created_at) AS first_transaction
FROM "transaction"
GROUP BY user_id;

-- Create a view to get the spend and benefit for each user for each casino
CREATE OR REPLACE VIEW user_casino_spend_benefit AS SELECT
    user_id,
    casino_id,
    SUM(cost) AS spend,
    SUM(benefit) AS benefit,
    (SUM(benefit) - SUM(cost)) / SUM(cost) AS average_bonus,
    COUNT(*) AS transactions,
    MAX(created_at) AS last_transaction,
    MIN(created_at) AS first_transaction
FROM "transaction"
GROUP BY user_id, casino_id;

-- Create a view to get the list of casinos and their names for each user
CREATE OR REPLACE VIEW user_casino_name AS SELECT
    user_id,
    casino_id,
    casino.name AS casino_name
FROM (user_casino JOIN casino ON user_casino.casino_id = casino.id);
