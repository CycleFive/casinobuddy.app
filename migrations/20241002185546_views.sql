---
--- Views for various summaries of the data and pages we want to display in the app.
---

-- Create a view to get the spend and benefit for each user
CREATE VIEW IF NOT EXISTS user_spend_benefit AS SELECT
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
CREATE VIEW IF NOT EXISTS user_casino_spend_benefit AS SELECT
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
