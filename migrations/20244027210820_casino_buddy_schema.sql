--- This is probably going to have a lot more in it.
CREATE TABLE IF NOT EXISTS casino (
    id                      INTEGER PRIMARY KEY AUTOINCREMENT,
    name                    TEXT NOT NULL,
    url                     TEXT NOT NULL,
    description             TEXT NOT NULL,
    daily_bonus             INTEGER NOT NULL,
    daily_limit             INTEGER NOT NULL,
    free_sweepstakes        INTEGER NOT NULL,
    prohibited_states       TEXT NOT NULL,
    prohibited_countries    TEXT NOT NULL,
    created_at              TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at              TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE TABLE IF NOT EXISTS "user" (
    id                  INTEGER PRIMARY KEY AUTOINCREMENT,
    email               TEXT NOT NULL,
    username            TEXT NOT NULL,
    avatar              TEXT,
    discord_id          TEXT,
    created_at          TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at          TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

--- Purchase data information, this is the data that will be used to calculate the user's total spend and total benefit.
CREATE TABLE IF NOT EXISTS purchase (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id         INTEGER NOT NULL,
    casino_id       INTEGER NOT NULL,
    cost            INTEGER NOT NULL,
    benefit         INTEGER NOT NULL,
    created_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    notes           TEXT,
    FOREIGN KEY (user_id) REFERENCES "user"(id),
    FOREIGN KEY (casino_id) REFERENCES casino(id)
);

-- Remption information. The received date is the date a person got the redepemtion in their account
CREATE TABLE IF NOT EXISTS redemption (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id         INTEGER NOT NULL,
    casino_id       INTEGER NOT NULL,
    amount          NUMERIC NOT NULL,
    created_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    received_at     TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES "user"(id),
    FOREIGN KEY (casino_id) REFERENCES casino(id)
);
CREATE TABLE IF NOT EXISTS user_casino (
    id                  INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id             INTEGER NOT NULL,
    casino_id           INTEGER NOT NULL,
    is_vip              BOOLEAN NOT NULL,
    is_verified         BOOLEAN NOT NULL,
    is_self_excluded    BOOLEAN NOT NULL,
    created_at          TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES "user"(id),
    FOREIGN KEY (casino_id) REFERENCES casino(id)
);

-- TODO: Replace the amount_other1, amount_other2, amount_other3, amount_other4 with a more generic approach.
CREATE TABLE IF NOT EXISTS daily_bonus (
    id                  INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id             INTEGER NOT NULL,
    casino_id           INTEGER NOT NULL,
    amount_sc           INTEGER NOT NULL,
    amount_gc           INTEGER NOT NULL,
    amount_other1       INTEGER NOT NULL,
    amount_other2       INTEGER NOT NULL,
    amount_other3       INTEGER NOT NULL,
    amount_other4       INTEGER NOT NULL,
    created_at          TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES "user"(id),
    FOREIGN KEY (casino_id) REFERENCES casino(id)
);
