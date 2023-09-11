-- Your SQL goes here

CREATE TABLE artists (
    id                      TEXT PRIMARY KEY, -- Artist name hashed with sha256
    name                    TEXT NOT NULL,
    created_at              TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at              TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE albums (
    id                      TEXT PRIMARY KEY,
    title                   TEXT NOT NULL,
    artist_id               TEXT NOT NULL REFERENCES artists(id) ON DELETE CASCADE,
    created_at              TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at              TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE tracks (
    id                      TEXT PRIMARY KEY,
    title                   TEXT NOT NULL,
    artist_id               TEXT REFERENCES artists(id) ON DELETE CASCADE,
    album_id                TEXT REFERENCES albums(id) ON DELETE CASCADE,
    duration                INT NOT NULL,
    year                    INT,
    track_number            INT,
    last_play               TIMESTAMP,
    plays                   INT NOT NULL DEFAULT 0,
    path                    TEXT NOT NULL,

    created_at              TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at              TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE features (
    id                      TEXT PRIMARY KEY,
    artist_id               TEXT NOT NULL REFERENCES artists(id) ON DELETE CASCADE,
    track_id                TEXT NOT NULL REFERENCES tracks(id) ON DELETE CASCADE,

    created_at              TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at              TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE scan_info (
    id                      SERIAL PRIMARY KEY,
    scan_start              TIMESTAMP NOT NULL,
    scan_end                TIMESTAMP NOT NULL,
    artists                 INT NOT NULL,
    albums                  INT NOT NULL,
    tracks                  INT NOT NULL
);

CREATE TABLE users (
    id                      TEXT PRIMARY KEY,
    name                    TEXT NOT NULL,
    password                TEXT NOT NULL,

    created_at              TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at              TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE sessions (
    id                      TEXT PRIMARY KEY,
    user_id                 TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    created_at              TIMESTAMP NOT NULL DEFAULT NOW(),
    expires_at              TIMESTAMP NOT NULL
);

CREATE TABLE favorites (
    id                      TEXT PRIMARY KEY,
    user_id                 TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    track_id                TEXT NOT NULL REFERENCES tracks(id) ON DELETE CASCADE,

    created_at              TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE OR REPLACE FUNCTION update_timestamp_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = now();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE OR REPLACE FUNCTION tracks_on_update()
RETURNS TRIGGER AS $$
BEGIN
    IF row(NEW.plays) IS DISTINCT FROM row(OLD.plays) THEN
        NEW.last_play = now();
    END IF;

    NEW.updated_at = now();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_artists
    BEFORE UPDATE ON artists
        FOR EACH ROW EXECUTE PROCEDURE update_timestamp_column();

CREATE TRIGGER update_albums
    BEFORE UPDATE ON albums
        FOR EACH ROW EXECUTE PROCEDURE update_timestamp_column();

CREATE TRIGGER update_tracks
    BEFORE UPDATE ON tracks
        FOR EACH ROW EXECUTE PROCEDURE tracks_on_update();

CREATE TRIGGER update_features
    BEFORE UPDATE ON features
        FOR EACH ROW EXECUTE PROCEDURE update_timestamp_column();

CREATE TRIGGER update_users
    BEFORE UPDATE ON users
        FOR EACH ROW EXECUTE PROCEDURE update_timestamp_column();
