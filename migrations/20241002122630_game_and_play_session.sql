-- Add migration script here
CREATE TABLE IF NOT EXISTS play_session (
    id                  INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    user_id             INTEGER NOT NULL,
    casino_id           INTEGER NOT NULL,
    game_id             INTEGER NOT NULL,
    beg_amount          NUMERIC NOT NULL,
    end_amount          NUMERIC NOT NULL,
    sc_per_spin         NUMERIC NOT NULL,
    play_date           TIMESTAMP NOT NULL,
    created_at          TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at          TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES user(id),
    FOREIGN KEY (casino_id) REFERENCES casino(id)
);

--- Create a table to store the information abou each game. There is an entry to each game on each site.
CREATE TABLE IF NOT EXISTS game (
    id                      INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    casino_id               INTEGER NOT NULL,
    developer_id            INTEGER NOT NULL,
    name                    TEXT NOT NULL,
    feature_hold_and_spin   BOOLEAN NOT NULL,
    feature_1               BOOLEAN NOT NULL,
    feature_2               BOOLEAN NOT NULL,
    feature_3               BOOLEAN NOT NULL,
    descriptions            TEXT,
    created_at              TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at              TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
