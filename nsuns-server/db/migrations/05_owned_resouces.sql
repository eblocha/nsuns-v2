ALTER TABLE profiles
ADD COLUMN owner_id UUID REFERENCES owners(id) ON DELETE CASCADE;

ALTER TABLE programs
ADD COLUMN owner_id UUID REFERENCES owners(id) ON DELETE CASCADE;

ALTER TABLE movements
ADD COLUMN owner_id UUID REFERENCES owners(id) ON DELETE CASCADE;

-- update movement constraint to only need unique names for the owner_id
ALTER TABLE movements
DROP CONSTRAINT unique_movement_name;
ALTER TABLE movements
ADD CONSTRAINT unique_movement_name UNIQUE (name, owner_id);

ALTER TABLE program_sets
ADD COLUMN owner_id UUID REFERENCES owners(id) ON DELETE CASCADE;

ALTER TABLE maxes
ADD COLUMN owner_id UUID REFERENCES owners(id) ON DELETE CASCADE;

ALTER TABLE reps
ADD COLUMN owner_id UUID REFERENCES owners(id) ON DELETE CASCADE;
