ALTER TABLE program_sets DROP CONSTRAINT unique_set;

ALTER TABLE program_sets
ADD CONSTRAINT unique_set UNIQUE (program_id, day, ordering);