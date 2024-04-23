-- Resource owners. Can be a logged-in user (long-term persistence), or anonymous (short-term persistence).
-- anonymous owners have an epiry date, after which they are deleted, along with their attached resources.
CREATE TABLE owners (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  expiry_date TIMESTAMPTZ
);

CREATE TABLE users (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  owner_id UUID NOT NULL REFERENCES owners(id) ON DELETE CASCADE,
  username VARCHAR NOT NULL,
  password_hash VARCHAR NOT NULL,
  CONSTRAINT unique_username UNIQUE (username),
  CONSTRAINT unique_owner UNIQUE (owner_id)
);

CREATE TABLE sessions (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  owner_id UUID NOT NULL REFERENCES owners(id) ON DELETE CASCADE,
  data BYTEA NOT NULL,
  expiry_date TIMESTAMPTZ NOT NULL
);