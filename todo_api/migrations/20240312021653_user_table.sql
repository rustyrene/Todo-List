-- Add migration script here
CREATE TABLE user_table (
  user_id UUID PRIMARY KEY NOT NULL UNIQUE,
  user_name TEXT NOT NULL UNIQUE,
  password TEXT NOT NULL
);
