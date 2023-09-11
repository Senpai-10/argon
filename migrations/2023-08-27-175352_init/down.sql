-- This file should undo anything in `up.sql`

DROP TABLE artists CASCADE;
DROP TABLE albums CASCADE;
DROP TABLE tracks CASCADE;
DROP TABLE features CASCADE;
DROP TABLE scan_info CASCADE;
DROP TABLE users CASCADE;
DROP TABLE sessions CASCADE;
DROP TABLE favorites CASCADE;

DROP FUNCTION update_timestamp_column CASCADE;
DROP FUNCTION tracks_on_update CASCADE;

