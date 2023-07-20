CREATE TABLE users (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  username VARCHAR NOT NULL,
  name VARCHAR,
  CONSTRAINT unique_user_username UNIQUE (username)
);
CREATE TABLE programs (
  id SERIAL PRIMARY KEY,
  name VARCHAR,
  description VARCHAR,
  created_on TIMESTAMP NOT NULL DEFAULT now(),
  -- user who owns the program
  owner UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE
);
ALTER TABLE users
ADD COLUMN default_program INTEGER REFERENCES programs(id) ON DELETE
SET NULL;
-- bench press, squat, etc.
CREATE TABLE movements (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  description VARCHAR,
  CONSTRAINT unique_movement_name UNIQUE (name)
);
CREATE TABLE "sets" (
  id SERIAL PRIMARY KEY,
  program_id INTEGER NOT NULL REFERENCES programs(id) ON DELETE CASCADE,
  movement_id INTEGER NOT NULL REFERENCES movements(id) ON DELETE CASCADE,
  -- sunday, monday, etc.
  day INTEGER NOT NULL CHECK (
    day >= 0
    AND day < 7
  ),
  ordering INTEGER NOT NULL,
  reps INTEGER CHECK (
    reps IS NULL
    OR reps >= 0
  ),
  -- represent "1+ reps"
  reps_is_minimum BOOLEAN DEFAULT false,
  description VARCHAR,
  CONSTRAINT unique_set UNIQUE (program_id, day, movement_id, ordering)
);
CREATE INDEX sets_by_program_id ON "sets"(program_id);
CREATE TABLE maxes (
  id SERIAL PRIMARY KEY,
  movement_id INTEGER NOT NULL REFERENCES movements(id) ON DELETE CASCADE,
  timestamp TIMESTAMP DEFAULT now(),
  amount DOUBLE PRECISION
);
CREATE TABLE reps (
  id SERIAL PRIMARY KEY,
  movement_id INTEGER NOT NULL REFERENCES movements(id) ON DELETE CASCADE,
  timestamp TIMESTAMP DEFAULT now(),
  amount INTEGER NOT NULL CHECK (amount >= 0)
);