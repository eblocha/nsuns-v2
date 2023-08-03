ALTER TABLE program_sets
ADD COLUMN program_id UUID REFERENCES programs(id) ON DELETE CASCADE;

UPDATE program_sets
SET program_id = (
    SELECT programs.id
    FROM programs
    WHERE program_sets.id = any(programs.set_ids_sunday)
      OR program_sets.id = any(programs.set_ids_monday)
      OR program_sets.id = any(programs.set_ids_tuesday)
      OR program_sets.id = any(programs.set_ids_wednesday)
      OR program_sets.id = any(programs.set_ids_thursday)
      OR program_sets.id = any(programs.set_ids_friday)
      OR program_sets.id = any(programs.set_ids_saturday)
  );

ALTER TABLE program_sets
ALTER COLUMN program_id
SET NOT NULL;

ALTER TABLE program_sets
ADD COLUMN day SMALLINT CHECK (
    day >= 0
    AND day < 7
  );

UPDATE program_sets
SET day = 0
FROM programs
WHERE program_sets.program_id = programs.id
  AND program_sets.id = any(programs.set_ids_sunday);

UPDATE program_sets
SET day = 1
FROM programs
WHERE program_sets.program_id = programs.id
  AND program_sets.id = any(programs.set_ids_monday);

UPDATE program_sets
SET day = 2
FROM programs
WHERE program_sets.program_id = programs.id
  AND program_sets.id = any(programs.set_ids_tuesday);

UPDATE program_sets
SET day = 3
FROM programs
WHERE program_sets.program_id = programs.id
  AND program_sets.id = any(programs.set_ids_wednesday);

UPDATE program_sets
SET day = 4
FROM programs
WHERE program_sets.program_id = programs.id
  AND program_sets.id = any(programs.set_ids_thursday);

UPDATE program_sets
SET day = 5
FROM programs
WHERE program_sets.program_id = programs.id
  AND program_sets.id = any(programs.set_ids_friday);

UPDATE program_sets
SET day = 6
FROM programs
WHERE program_sets.program_id = programs.id
  AND program_sets.id = any(programs.set_ids_saturday);

ALTER TABLE program_sets
ALTER COLUMN day
SET NOT NULL;