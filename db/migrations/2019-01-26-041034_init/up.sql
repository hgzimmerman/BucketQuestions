CREATE TABLE users (
    uuid UUID PRIMARY KEY NOT NULL Default gen_random_uuid(),
    google_user_id VARCHAR NOT NULL
);

