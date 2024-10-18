-- Enable the uuid-ossp extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

--- This is probably going to have a lot more in it.
CREATE TABLE IF NOT EXISTS casino (
    id                      UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name                    VARCHAR(2048) NOT NULL,
    url                     VARCHAR(2048) NOT NULL,
    description             VARCHAR(2048) NOT NULL,
    created_at              TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at              TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
-- Going to need another table for the metadata of the casino in order to maintain flexibility
--   daily_bonus             BIGINT NOT NULL,
--   daily_limit             BIGINT NOT NULL,
--   free_sweepstakes        BOOLEAN NOT NULL,
--   prohibited_states       TEXT,
--   prohibited_countries    TEXT,

CREATE TABLE IF NOT EXISTS "user" (
    id                  UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    oauth_id            TEXT NOT NULL UNIQUE,
    created_at          TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at          TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

--- Transaction data information, this is the data that will be used to calculate the user's total spend and total benefit.
CREATE TABLE IF NOT EXISTS "transaction" (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id         UUID NOT NULL,
    casino_id       UUID NOT NULL,
    cost            NUMERIC NOT NULL,
    benefit         NUMERIC NOT NULL,
    created_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    notes           VARCHAR(2048),
    FOREIGN KEY (user_id) REFERENCES "user"(id) ON DELETE CASCADE,
    FOREIGN KEY (casino_id) REFERENCES casino(id) ON DELETE CASCADE
);

-- Remption information. The received date is the date a person got the redepemtion in their account
CREATE TABLE IF NOT EXISTS redemption (
    id                      UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id                 UUID NOT NULL,
    casino_id               UUID NOT NULL,
    amount                  NUMERIC NOT NULL,
    created_at              TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    received_at             TIMESTAMP,
    FOREIGN KEY (user_id)   REFERENCES "user"(id),
    FOREIGN KEY (casino_id) REFERENCES casino(id)
);

--- User casino information, links the user to each casino they have an account with.
CREATE TABLE IF NOT EXISTS user_casino (
    id                      UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id                 UUID NOT NULL,
    casino_id               UUID NOT NULL,
    is_vip                  BOOLEAN NOT NULL DEFAULT FALSE,
    is_verified             BOOLEAN NOT NULL DEFAULT FALSE,
    is_self_excluded        BOOLEAN NOT NULL DEFAULT FALSE,
    created_at              TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id)   REFERENCES "user"(id) ON DELETE CASCADE,
    FOREIGN KEY (casino_id) REFERENCES casino(id) ON DELETE CASCADE
);

-- TODO: Replace the amount_other1, amount_other2, amount_other3, amount_other4 with a more generic approach.
CREATE TABLE IF NOT EXISTS daily_bonus (
    id                      UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id                 UUID NOT NULL,
    casino_id               UUID NOT NULL,
    amount_sc               NUMERIC NOT NULL,
    amount_gc               NUMERIC NOT NULL,
    amount_other1           NUMERIC NOT NULL,
    amount_other2           NUMERIC NOT NULL,
    amount_other3           NUMERIC NOT NULL,
    amount_other4           NUMERIC NOT NULL,
    created_at              TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id)   REFERENCES "user"(id) ON DELETE CASCADE,
    FOREIGN KEY (casino_id) REFERENCES casino(id) ON DELETE CASCADE
);
