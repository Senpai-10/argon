-- Your SQL goes here

CREATE TABLE albums (
    title TEXT NOT NULL PRIMARY KEY
);

CREATE TABLE artists (
    name TEXT NOT NULL PRIMARY KEY
);

CREATE TABLE songs (
    id TEXT NOT NULL PRIMARY KEY,
    title TEXT NOT NULL,
    artist_name TEXT NOT NULL,
    album_title TEXT NOT NULL,
    length INT NOT NULL,
    --genre TEXT NOT NULL,
    year INT,
    track INT,
    -- Path to file
    path TEXT NOT NULL,

    FOREIGN KEY (album_title)
        REFERENCES albums(title)
        ON DELETE CASCADE

    FOREIGN KEY (artist_name)
        REFERENCES artists(name)
        ON DELETE CASCADE
);

CREATE TABLE features (
    id TEXT NOT NULL PRIMARY KEY,
    artist_name TEXT NOT NULL,
    song_id TEXT NOT NULL,

    FOREIGN KEY (artist_name)
        REFERENCES artists(name)
        ON DELETE CASCADE

    FOREIGN KEY (song_id)
        REFERENCES songs(id)
        ON DELETE CASCADE
);
