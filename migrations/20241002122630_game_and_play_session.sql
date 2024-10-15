-- Add migration script here
CREATE TABLE IF NOT EXISTS play_session (
    id                      UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id                 UUID NOT NULL,
    casino_id               UUID NOT NULL,
    game_id                 UUID NOT NULL,
    beg_amount              NUMERIC NOT NULL,
    end_amount              NUMERIC NOT NULL,
    sc_per_spin             NUMERIC NOT NULL,
    num_spins               NUMERIC NOT NULL,
    play_date               TIMESTAMP NOT NULL,
    created_at              TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at              TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id)   REFERENCES "user"(id),
    FOREIGN KEY (casino_id) REFERENCES casino(id)
);

--- Create a table to store the information about each developer.
CREATE TABLE IF NOT EXISTS developer (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name        TEXT NOT NULL,
    created_at  TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at  TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
); 


--- Create a table to store the information abou each game. There is an entry to each game on each site.
CREATE TABLE IF NOT EXISTS game (
    id                          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    casino_id                   UUID NOT NULL,
    developer_id                UUID NOT NULL,
    name                        TEXT NOT NULL,
    feature_hold_and_spin       BOOLEAN NOT NULL,
    feature_1                   BOOLEAN NOT NULL,
    feature_2                   BOOLEAN NOT NULL,
    feature_3                   BOOLEAN NOT NULL,
    descriptions                TEXT,
    created_at                  TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at                  TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (casino_id)     REFERENCES casino(id) ON DELETE CASCADE,
    FOREIGN KEY (developer_id)  REFERENCES developer(id) ON DELETE CASCADE
);
