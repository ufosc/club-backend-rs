-- Your SQL goes here
CREATE TABLE users (
    user_id SERIAL PRIMARY KEY,
    username VARCHAR NOT NULL,
    email VARCHAR,
    password_hash VARCHAR
);
