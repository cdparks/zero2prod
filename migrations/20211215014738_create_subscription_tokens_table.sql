CREATE TABLE tokens (
  token TEXT NOT NULL PRIMARY KEY,
  subscription_id uuid NOT NULL REFERENCES subscriptions (id)
);
