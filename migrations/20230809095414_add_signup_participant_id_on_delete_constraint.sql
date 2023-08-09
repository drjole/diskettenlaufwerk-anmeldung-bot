BEGIN;

ALTER TABLE signups
DROP CONSTRAINT IF EXISTS signups_participant_id_fkey;

ALTER TABLE signups
ADD CONSTRAINT signups_participant_id_fkey FOREIGN KEY (participant_id)
REFERENCES participants(id) ON DELETE CASCADE;

COMMIT;
