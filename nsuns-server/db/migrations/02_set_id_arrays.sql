ALTER TABLE programs
ADD COLUMN set_ids_sunday UUID [] DEFAULT '{}';

ALTER TABLE programs
ADD COLUMN set_ids_monday UUID [] DEFAULT '{}';

ALTER TABLE programs
ADD COLUMN set_ids_tuesday UUID [] DEFAULT '{}';

ALTER TABLE programs
ADD COLUMN set_ids_wednesday UUID [] DEFAULT '{}';

ALTER TABLE programs
ADD COLUMN set_ids_thursday UUID [] DEFAULT '{}';

ALTER TABLE programs
ADD COLUMN set_ids_friday UUID [] DEFAULT '{}';

ALTER TABLE programs
ADD COLUMN set_ids_saturday UUID [] DEFAULT '{}';

UPDATE programs
SET set_ids_sunday = array(
    SELECT id
    FROM program_sets
    WHERE program_id = programs.id
      AND day = 0
  );

UPDATE programs
SET set_ids_monday = array(
    SELECT id
    FROM program_sets
    WHERE program_id = programs.id
      AND day = 1
  );

UPDATE programs
SET set_ids_tuesday = array(
    SELECT id
    FROM program_sets
    WHERE program_id = programs.id
      AND day = 2
  );

UPDATE programs
SET set_ids_wednesday = array(
    SELECT id
    FROM program_sets
    WHERE program_id = programs.id
      AND day = 3
  );

UPDATE programs
SET set_ids_thursday = array(
    SELECT id
    FROM program_sets
    WHERE program_id = programs.id
      AND day = 4
  );

UPDATE programs
SET set_ids_friday = array(
    SELECT id
    FROM program_sets
    WHERE program_id = programs.id
      AND day = 5
  );

UPDATE programs
SET set_ids_saturday = array(
    SELECT id
    FROM program_sets
    WHERE program_id = programs.id
      AND day = 6
  );

ALTER TABLE programs
ALTER COLUMN set_ids_sunday
SET NOT NULL;

ALTER TABLE programs
ALTER COLUMN set_ids_monday
SET NOT NULL;

ALTER TABLE programs
ALTER COLUMN set_ids_tuesday
SET NOT NULL;

ALTER TABLE programs
ALTER COLUMN set_ids_wednesday
SET NOT NULL;

ALTER TABLE programs
ALTER COLUMN set_ids_thursday
SET NOT NULL;

ALTER TABLE programs
ALTER COLUMN set_ids_friday
SET NOT NULL;

ALTER TABLE programs
ALTER COLUMN set_ids_saturday
SET NOT NULL;

ALTER TABLE program_sets DROP CONSTRAINT unique_set;

ALTER TABLE program_sets DROP COLUMN program_id;

ALTER TABLE program_sets DROP COLUMN day;

ALTER TABLE program_sets DROP COLUMN ordering;