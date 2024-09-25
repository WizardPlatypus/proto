CREATE TABLE IF NOT EXISTS users (
       id UUID PRIMARY KEY NOT NULL,
       nickname VARCHAR(29) UNIQUE NOT NULL,
       email TEXT NOT NULL,
       phc TEXT,
       avatar BYTEA,
       info TEXT,
       joined TIMESTAMPTZ NOT NULL
);
