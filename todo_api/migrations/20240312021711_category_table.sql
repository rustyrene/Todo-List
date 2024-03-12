-- Add migration script here
CREATE TABLE category_table (
  category_id UUID PRIMARY KEY NOT NULL UNIQUE,
  category_name TEXT NOT NULL,
  user_id UUID REFERENCES user_table(user_id) ON DELETE CASCADE,
  UNIQUE (user_id, category_name)
);    
