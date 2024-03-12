-- Add migration script here
CREATE TABLE todo_table (
  todo_id UUID PRIMARY KEY NOT NULL UNIQUE,
  todo_name TEXT NOT NULL,
  id_done BOOLEAN NOT NULL,
  user_id UUID REFERENCES user_table(user_id) ON DELETE CASCADE,
  category_id UUID REFERENCES category_table(category_id) ON DELETE CASCADE
);
