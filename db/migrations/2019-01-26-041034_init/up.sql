CREATE TABLE users (
    uuid UUID PRIMARY KEY NOT NULL Default gen_random_uuid(),
    google_user_id INTEGER NOT NULL
);

-- CREATE TABLE events (
--     uuid UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
--     user_uuid UUID NOT NULL REFERENCES users(uuid) ON DELETE CASCADE,
--     title VARCHAR NOT NULL,
--     text VARCHAR NOT NULL,
--     start_at TIMESTAMP NOT NULL,
--     stop_at TIMESTAMP NOT NULL
-- );
--
--
-- CREATE TABLE stocks (
--     uuid UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
--     symbol VARCHAR NOT NULL,
--     stock_name VARCHAR NOT NULL
-- );
--
-- CREATE TABLE stock_transactions (
--     uuid UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
--     user_uuid UUID NOT NULL REFERENCES users(uuid) ON DELETE CASCADE,
--     stock_uuid UUID NOT NULL REFERENCES stocks(uuid) ON DELETE CASCADE,
--     quantity INTEGER NOT NULL,
--     price_of_stock_at_time_of_trading FLOAT NOT NULL,
--     record_time TIMESTAMP NOT NULL
-- );
--
-- CREATE TABLE adaptive_health (
--     uuid UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
--     available_servers INTEGER NOT NULL CHECK (available_servers > 0),
--     load INTEGER NOT NULL CHECK (load > 0),
--     did_serve boolean NOT NULL,
--     time_recorded TIMESTAMP NOT NULL
-- )
