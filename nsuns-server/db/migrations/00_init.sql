CREATE TABLE profiles (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  name VARCHAR NOT NULL
);
CREATE TABLE programs (
  id SERIAL PRIMARY KEY,
  name VARCHAR,
  description VARCHAR,
  created_on TIMESTAMP NOT NULL DEFAULT now(),
  -- profile that owns the program
  owner UUID NOT NULL REFERENCES profiles(id) ON DELETE CASCADE
);
-- bench press, squat, etc.
CREATE TABLE movements (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  description VARCHAR,
  CONSTRAINT unique_movement_name UNIQUE (name)
);
CREATE TABLE program_sets (
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
  amount DOUBLE PRECISION NOT NULL,
  percentage_of_max INTEGER REFERENCES movements(id) ON DELETE SET NULL,
  CONSTRAINT unique_set UNIQUE (program_id, day, movement_id, ordering)
);
CREATE INDEX sets_by_program_id ON program_sets(program_id);
CREATE TABLE maxes (
  id SERIAL PRIMARY KEY,
  profile_id UUID NOT NULL REFERENCES profiles(id) ON DELETE CASCADE,
  movement_id INTEGER NOT NULL REFERENCES movements(id) ON DELETE CASCADE,
  timestamp TIMESTAMP DEFAULT now(),
  amount DOUBLE PRECISION NOT NULL CHECK (amount >= 0)
);
CREATE INDEX maxes_by_profile_id ON maxes(profile_id);
CREATE TABLE reps (
  id SERIAL PRIMARY KEY,
  profile_id UUID NOT NULL REFERENCES profiles(id) ON DELETE CASCADE,
  movement_id INTEGER NOT NULL REFERENCES movements(id) ON DELETE CASCADE,
  timestamp TIMESTAMP DEFAULT now(),
  amount INTEGER CHECK (amount IS NULL OR amount >= 0)
);
CREATE INDEX reps_by_profile_id ON reps(profile_id);