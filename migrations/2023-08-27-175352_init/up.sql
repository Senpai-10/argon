-- Your SQL goes here

CREATE TABLE artists (
    id                      TEXT PRIMARY KEY, -- Artist name hashed with sha256
    name                    TEXT NOT NULL,
    created_at              TIMESTAMP NOT NULL,
    updated_at              TIMESTAMP
);

CREATE TABLE albums (
    id                      TEXT PRIMARY KEY,
    title                   TEXT NOT NULL,
    artist_id               TEXT NOT NULL REFERENCES artists(id) ON DELETE CASCADE,
    created_at              TIMESTAMP NOT NULL,
    updated_at              TIMESTAMP
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

    created_at              TIMESTAMP NOT NULL,
    updated_at              TIMESTAMP
);

CREATE TABLE features (
    id                      TEXT PRIMARY KEY,
    artist_id               TEXT NOT NULL REFERENCES artists(id) ON DELETE CASCADE,
    track_id                TEXT NOT NULL REFERENCES tracks(id) ON DELETE CASCADE,

    created_at              TIMESTAMP NOT NULL,
    updated_at              TIMESTAMP
);

CREATE TABLE scan_info (
    id                      SERIAL PRIMARY KEY,
    scan_start              TIMESTAMP NOT NULL,
    scan_end                TIMESTAMP NOT NULL,
    artists                 INT NOT NULL,
    albums                  INT NOT NULL,
    tracks                  INT NOT NULL
);

