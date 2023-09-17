use crate::models::albums::NewAlbum;
use crate::models::artists::NewArtist;
use crate::models::features::NewFeature;
use crate::models::scan_info::{NewScanInfo, ScanInfo};
use crate::models::tracks::NewTrack;
use crate::schema::*;
use chrono::{NaiveDateTime, Utc};
use diesel::dsl::{exists, select};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use id3::TagLike;
use mpeg_audio_header::{Header, ParseMode};
use nanoid::nanoid;
use std::env;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// check if this is the first time the program starts
pub fn is_first_run(conn: &mut PgConnection) -> bool {
    let count = scan_info::table
        .count()
        .get_result::<i64>(conn)
        .unwrap_or(0);

    count == 0
}

fn track_exists(conn: &mut PgConnection, id: &String) -> bool {
    select(exists(tracks::table.filter(tracks::id.eq(id))))
        .get_result::<bool>(conn)
        .unwrap()
}

fn album_exists(conn: &mut PgConnection, title: &String, artist_id: &String) -> bool {
    select(exists(
        albums::table
            .filter(albums::title.eq(title))
            .filter(albums::artist_id.eq(artist_id)),
    ))
    .get_result::<bool>(conn)
    .unwrap()
}

fn artist_exists(conn: &mut PgConnection, id: &String) -> bool {
    select(exists(artists::table.filter(artists::id.eq(id))))
        .get_result::<bool>(conn)
        .unwrap()
}

fn feature_exists(conn: &mut PgConnection, artist_id: String, track_id: String) -> bool {
    select(exists(
        features::table
            .filter(features::artist_id.eq(artist_id))
            .filter(features::track_id.eq(track_id)),
    ))
    .get_result::<bool>(conn)
    .unwrap()
}

#[derive(Default)]
struct Counter {
    artists: i32,
    albums: i32,
    tracks: i32,
}

pub struct Scanner {
    pub id: String,
    conn: PgConnection,
    counter: Counter,
    scan_start: NaiveDateTime,
}

impl Scanner {
    pub fn new(conn: PgConnection) -> Self {
        Self {
            id: nanoid!(),
            conn,
            counter: Counter::default(),
            scan_start: Utc::now().naive_utc(),
        }
    }

    fn get_lock_file(&self) -> PathBuf {
        env::temp_dir().join(".argon-scanner-lock")
    }

    pub fn is_locked(&self) -> bool {
        self.get_lock_file().exists()
    }

    fn lock(&self) {
        let file = self.get_lock_file();

        if !file.exists() {
            _ = std::fs::write(file, "");
        }
    }

    fn unlock(&self) {
        let file = self.get_lock_file();

        if file.exists() {
            _ = std::fs::remove_file(file);
        }
    }

    fn record_scan_start(&mut self) -> Result<ScanInfo, diesel::result::Error> {
        let new_scan_info = NewScanInfo {
            id: self.id.clone(),
            scan_start: self.scan_start,
            scan_end: None,
            is_done: false,
            artists: self.counter.artists,
            albums: self.counter.albums,
            tracks: self.counter.tracks,
        };

        diesel::insert_into(scan_info::table)
            .values(&new_scan_info)
            .get_result::<ScanInfo>(&mut self.conn)
    }

    fn record_scan_end(&mut self) -> Result<ScanInfo, diesel::result::Error> {
        let scan_end = Utc::now().naive_utc();

        diesel::update(scan_info::table.filter(scan_info::id.eq(&self.id)))
            .set((
                scan_info::scan_end.eq(Some(scan_end)),
                scan_info::is_done.eq(true),
                scan_info::artists.eq(self.counter.artists),
                scan_info::albums.eq(self.counter.albums),
                scan_info::tracks.eq(self.counter.tracks),
            ))
            .get_result::<ScanInfo>(&mut self.conn)
    }

    pub fn start(&mut self) {
        if self.is_locked() {
            return;
        }

        let music_dir: PathBuf = match env::var("ARGON_MUSIC_LIB") {
            Ok(v) => v.into(),
            Err(_) => {
                let home = dirs::home_dir().unwrap();

                home.join("Music")
            }
        };

        if !music_dir.exists() {
            error!("Music dir does not exists! {}", music_dir.display());
            return;
        }

        self.lock();

        _ = self.record_scan_start();

        info!("Scanning Music library! '{}'", music_dir.display());

        for entry in WalkDir::new(music_dir).into_iter().flatten() {
            let file_path = entry.path();

            if !file_path.is_file() || file_path.extension().unwrap() != "mp3" {
                continue;
            }

            self.process_file(file_path);
        }

        self.unlock();

        match self.record_scan_end() {
            Ok(_) => {
                info!("Scan info saved!");
            }
            Err(e) => {
                error!("Failed to save scan info to database!, {e}")
            }
        };
    }

    fn process_file(&mut self, file_path: &Path) {
        let id = sha256::try_digest(file_path).unwrap();

        if track_exists(&mut self.conn, &id) {
            return;
        }

        let mut features_insert_queue: Vec<NewFeature> = Vec::new();

        let mut new_track = NewTrack {
            id,
            title: "Untitled".to_string(),
            artist_id: None,
            album_id: None,
            duration: 0,
            year: None,
            track_number: None,
            path: file_path.to_str().unwrap().to_string(),
        };

        self.counter.tracks += 1;

        let tag = id3::Tag::read_from_path(file_path).unwrap();

        if let Some(title) = tag.title() {
            new_track.title = title.to_string()
        }

        if let Some(artists) = tag.artists() {
            for (index, artist) in artists.into_iter().enumerate() {
                let artist_name_hash = sha256::digest(artist);

                // Create artist if does not exists
                if !artist_exists(&mut self.conn, &artist_name_hash) {
                    let new_artist = NewArtist {
                        id: artist_name_hash.clone(),
                        name: artist.to_string(),
                    };

                    match diesel::insert_into(artists::table)
                        .values(new_artist)
                        .execute(&mut self.conn)
                    {
                        Ok(_) => {
                            info!("Added new artist '{}' to database", &artist);
                            self.counter.artists += 1;
                        }
                        Err(e) => {
                            error!("Failed to add artist to database!, {e}");
                        }
                    };
                }

                if index == 0 {
                    new_track.artist_id = Some(artist_name_hash);
                    continue;
                }

                if !feature_exists(&mut self.conn, artist.to_string(), new_track.id.clone()) {
                    let new_feature = NewFeature {
                        id: nanoid!(),
                        artist_id: artist_name_hash.clone(),
                        track_id: new_track.id.clone(),
                    };

                    features_insert_queue.push(new_feature);
                }
            }

            if let Some(album) = tag.album() {
                if !album_exists(
                    &mut self.conn,
                    &album.to_string(),
                    &new_track.artist_id.clone().unwrap(),
                ) {
                    let new_album = NewAlbum {
                        id: nanoid!(),
                        title: album.to_string(),
                        artist_id: new_track.artist_id.clone().unwrap(),
                    };

                    match diesel::insert_into(albums::table)
                        .values(&new_album)
                        .execute(&mut self.conn)
                    {
                        Ok(_) => {
                            info!("Added new album '{}' to database", new_album.title);
                            self.counter.albums += 1;
                        }
                        Err(e) => {
                            error!("Failed to add album to database!, {e}");
                        }
                    }

                    new_track.album_id = Some(new_album.id);
                }
            }
        }

        if let Ok(header) = Header::read_from_path(file_path, ParseMode::PreferVbrHeaders) {
            new_track.duration = header.total_duration.as_secs() as i32
        }

        if let Some(year) = tag.year() {
            new_track.year = Some(year)
        }

        if let Some(track_num) = tag.track() {
            new_track.track_number = Some(track_num as i32)
        }

        match diesel::insert_into(tracks::table)
            .values(&new_track)
            .execute(&mut self.conn)
        {
            Ok(_) => {
                info!(
                    "Added new track '{}' to database",
                    if new_track.title == "Untitled" {
                        &new_track.path
                    } else {
                        &new_track.title
                    }
                );
            }
            Err(e) => {
                error!("Failed to add track to database!, {e}");
            }
        }

        if !features_insert_queue.is_empty() {
            match diesel::insert_into(features::table)
                .values(&features_insert_queue)
                .execute(&mut self.conn)
            {
                Ok(_) => {
                    info!(
                        "Added new featured artists({}) on '{}' to database",
                        features_insert_queue.len(),
                        new_track.title,
                    );
                }
                Err(e) => {
                    error!("Failed to add featured artist to database!, {e}");
                }
            }
        }
    }
}
