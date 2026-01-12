-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users (
                       id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
                       telegram_id BIGINT NOT NULL UNIQUE,
                       faculty TEXT NOT NULL,
                       group_name TEXT NOT NULL,
                       created_at TIMESTAMP NOT NULL DEFAULT NOW()
);