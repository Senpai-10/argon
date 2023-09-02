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
    plays                   INT NOT NULL,
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

CREATE OR REPLACE FUNCTION update_timestamp_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = now();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_artists_updated_at
    BEFORE UPDATE ON artists
        FOR EACH ROW EXECUTE PROCEDURE update_timestamp_column();

CREATE TRIGGER update_albums_updated_at
    BEFORE UPDATE ON albums
        FOR EACH ROW EXECUTE PROCEDURE update_timestamp_column();

CREATE TRIGGER update_tracks_updated_at
    BEFORE UPDATE ON tracks
        FOR EACH ROW EXECUTE PROCEDURE update_timestamp_column();

CREATE TRIGGER update_features_updated_at
    BEFORE UPDATE ON features
        FOR EACH ROW EXECUTE PROCEDURE update_timestamp_column();

