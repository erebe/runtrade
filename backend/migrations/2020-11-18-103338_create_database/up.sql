CREATE TABLE users
(
    id        SERIAL PRIMARY KEY,
    name      TEXT NOT NULL CHECK (name <> ''),
    email     TEXT NOT NULL CHECK (email <> ''),
    contact_1 TEXT CHECK (contact_1 <> ''),
    contact_2 TEXT CHECK (contact_2 <> ''),
    contact_3 TEXT CHECK (contact_3 <> '')
);

CREATE TYPE EVENT_TYPE AS ENUM ('run', 'trail', 'cross', 'triathlon', 'ironman', 'bike', 'other');

CREATE TABLE events
(
    id           SERIAL PRIMARY KEY,
    name         TEXT       NOT NULL CHECK (name <> '') UNIQUE,
    event_type   EVENT_TYPE NOT NULL,
    localisation TEXT       NOT NULL CHECK (localisation <> ''),
    event_date   TIMESTAMP  NOT NULL,
    event_link   TEXT       NOT NULL

);

CREATE TYPE INSCRIPTION_INTENT AS ENUM ('Buy', 'Sell');

CREATE TABLE inscriptions
(
    id         SERIAL PRIMARY KEY,
    user_id    SERIAL REFERENCES users (id) ON DELETE CASCADE,
    event_id   SERIAL REFERENCES events (id) ON DELETE CASCADE,
    distance   TEXT               NOT NULL CHECK (distance <> ''),
    price      REAL               NOT NULL CHECK (price >= 0.0),
    intent     INSCRIPTION_INTENT NOT NULL,
    created_at timestamp          NOT NULL
);

