CREATE TABLE bq_user (
    uuid UUID PRIMARY KEY NOT NULL Default gen_random_uuid(),
    google_user_id VARCHAR NOT NULL,
    google_name VARCHAR
);

