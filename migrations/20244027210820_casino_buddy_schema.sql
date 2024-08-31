--- This is probably going to have a lot more in it.
CREATE TABLE IF NOT EXISTS sites (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    url TEXT NOT NULL,
    description TEXT NOT NULL,
    daily_bonus INTEGER NOT NULL,
    daily_limit INTEGER NOT NULL,
    free_sweepstakes INTEGER NOT NULL,
    prohibited_states TEXT NOT NULL,
    prohibited_countries TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS "user" (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
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

--- Purchase data information, this is the data that will be used to calculate the user's total spend and total benefit.
CREATE TABLE IF NOT EXISTS purchase (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id     INTEGER NOT NULL,
    site_id     INTEGER NOT NULL,
    cost        INTEGER NOT NULL,
    benefit     INTEGER NOT NULL,
    created_at  TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    notes       TEXT,
    FOREIGN KEY (user_id) REFERENCES "user"(id),
    FOREIGN KEY (site_id) REFERENCES sites(id)
);
CREATE TABLE IF NOT EXISTS redemption (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    site_id INTEGER NOT NULL,
    amount NUMERIC NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    received_at TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES "user"(id),
    FOREIGN KEY (site_id) REFERENCES sites(id)
);
CREATE TABLE IF NOT EXISTS user_site (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    site_id INTEGER NOT NULL,
    is_vip BOOLEAN NOT NULL,
    is_verified BOOLEAN NOT NULL,
    is_self_excluded BOOLEAN NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES "user"(id),
    FOREIGN KEY (site_id) REFERENCES sites(id)
);
CREATE TABLE IF NOT EXISTS daily_bonus (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    site_id INTEGER NOT NULL,
    amount_sc INTEGER NOT NULL,
    amount_gc INTEGER NOT NULL,
    amount_other1 INTEGER NOT NULL,
    amount_other2 INTEGER NOT NULL,
    amount_other3 INTEGER NOT NULL,
    amount_other4 INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES "user"(id),
    FOREIGN KEY (site_id) REFERENCES sites(id)
);
