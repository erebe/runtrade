CREATE EXTENSION IF NOT EXISTS pg_trgm;
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
SET TIMEZONE = 'UTC';
SET client_encoding = 'UTF8';

CREATE TABLE users
(
    id          uuid PRIMARY KEY   DEFAULT uuid_generate_v4(),
    name        TEXT      NOT NULL CHECK (name <> ''),
    email       TEXT      NOT NULL CHECK (email <> ''),
    contact     TEXT      NOT NULL,
    last_logged TIMESTAMP NOT NULL DEFAULT now()
);

CREATE UNIQUE INDEX idx_users_email_btree ON users (email);

INSERT INTO users
VALUES ('ffffffff-ffff-ffff-ffff-ffffffffffff', 'admin', 'runtrade@erebe.eu', 'https://m.me/erebe.dellu.42', to_timestamp(0));


CREATE TYPE EVENT_TYPE AS ENUM ('run', 'trail', 'cross', 'triathlon', 'ironman', 'bike', 'other');

CREATE TABLE events
(
    id           SERIAL PRIMARY KEY,
    name         TEXT       NOT NULL CHECK (name <> '') UNIQUE,
    event_type   EVENT_TYPE NOT NULL,
    localisation TEXT       NOT NULL CHECK (localisation <> ''),
    event_date   TIMESTAMP  NOT NULL,
    event_link   TEXT       NOT NULL,
    created_at   TIMESTAMP  NOT NULL,
    user_id      uuid REFERENCES users (id) ON DELETE SET DEFAULT

);
CREATE INDEX idx_events_name_gin ON events USING gin (name gin_trgm_ops);
CREATE INDEX idx_events_localisation_gin ON events USING gin (localisation gin_trgm_ops);
CREATE INDEX idx_events_eventlink_gin ON events USING gin (event_link gin_trgm_ops);

CREATE TYPE INSCRIPTION_INTENT AS ENUM ('buy', 'sell');
CREATE TYPE GENDER AS ENUM ('man', 'woman', 'other');

CREATE TABLE inscriptions
(
    id         SERIAL PRIMARY KEY,
    user_id    uuid REFERENCES users (id) ON DELETE CASCADE,
    event_id   SERIAL REFERENCES events (id) ON DELETE CASCADE,
    category   TEXT               NOT NULL CHECK,
    price      REAL               NOT NULL CHECK (price >= 0.0),
    currency   VARCHAR(1)         NOT NULL CHECK (currency <> ''),
    intent     INSCRIPTION_INTENT NOT NULL,
    created_at timestamp          NOT NULL,
    note       TEXT               NOT NULL,
    gender     GENDER             NOT NULL
);

