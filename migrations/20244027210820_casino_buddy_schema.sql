--- This is probably going to have a lot more in it.
CREATE TABLE IF NOT EXISTS sites (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    url TEXT NOT NULL,
    description TEXT NOT NULL,
    daily_bonus INT NOT NULL,
    daily_limit INT NOT NULL,
    free_sweepstakes INT NOT NULL,
    prohibited_states TEXT NOT NULL,
    prohibited_countries TEXT NOT NULL,
);
--- User table, what oauth do we want to support? Google, Facebook, Discord, What else?
CREATE TABLE IF NOT EXISTS "user" (
    id SERIAL PRIMARY KEY,
    oauth_id TEXT NOT NULL,
    email TEXT NOT NULL,
    username TEXT NOT NULL,
    avatar TEXT NOT NULL,
    discord_id TEXT,
    google_id TEXT,
    facebook_id TEXT,
    accept_tos BOOLEAN NOT NULL,
    accept_privacy BOOLEAN NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
--- Buy/Spend/Purchase?
CREATE TABLE IF NOT EXISTS transactions (
    id SERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL,
    site_id INT NOT NULL,
    cost INT NOT NULL,
    benefit INT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_transactions_user_id FOREIGN KEY (user_id) REFERENCES "user"(id),
    CONSTRAINT fk_transactions_site_id FOREIGN KEY (site_id) REFERENCES sites(id)
);
--- Redemptions
CREATE TABLE IF NOT EXISTS redemptions (
    id SERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL,
    site_id INT NOT NULL,
    amount INT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    received_at TIMESTAMP,
    CONSTRAINT fk_redemptions_user_id FOREIGN KEY (user_id) REFERENCES "user"(id),
    CONSTRAINT fk_redemptions_site_id FOREIGN KEY (site_id) REFERENCES sites(id)
);
--- User / site relationship (self exclusion, verification, VIP, etc)
CREATE TABLE IF NOT EXISTS user_site (
    id SERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL,
    site_id INT NOT NULL,
    is_vip BOOLEAN NOT NULL,
    is_verified BOOLEAN NOT NULL,
    is_self_excluded BOOLEAN NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_user_site_user_id FOREIGN KEY (user_id) REFERENCES "user"(id),
    CONSTRAINT fk_user_site_site_id FOREIGN KEY (site_id) REFERENCES sites(id)
);
--- Daily bonus claims
CREATE TABLE IF NOT EXISTS daily_bonus (
    id SERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL,
    site_id INT NOT NULL,
    amount_sc INT NOT NULL,
    amount_gc INT NOT NULL,
    --- Maybe do this as a JSONB field?
    amount_other1 INT NOT NULL,
    amount_other2 INT NOT NULL,
    amount_other3 INT NOT NULL,
    amount_other4 INT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_daily_bonus_user_id FOREIGN KEY (user_id) REFERENCES "user"(id),
    CONSTRAINT fk_daily_bonus_site_id FOREIGN KEY (site_id) REFERENCES sites(id)
);