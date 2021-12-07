CREATE TABLE subscriptions (
  id uuid PRIMARY KEY NOT NULL,
  email TEXT NOT NULL UNIQUE,
  name TEXT NOT NULL,
  created_at timestamptz NOT NULL
);
