mod extract_metadata;

use crate::models::albums::NewAlbum;
use crate::models::artists::NewArtist;
use crate::models::features::NewFeature;
use crate::models::scan_info::{NewScanInfo, ScanInfo};
use crate::models::tracks::{NewTrack, Track};
use crate::schema::*;
use chrono::{NaiveDateTime, Utc};
use diesel::dsl::{exists, select};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use extract_metadata::{extract_metadata, Metadata};
use nanoid::nanoid;
use std::env;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

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

#[derive(AsChangeset, Debug, Default)]
#[diesel(table_name = tracks)]
struct UpdateTrackForm {
    title: Option<String>,
    artist_id: Option<String>,
    album_id: Option<String>,
    duration: Option<i32>,
    year: Option<i32>,
    track_number: Option<i32>,
    path: Option<String>,
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

// TODO: Refactor
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

            let id = sha256::digest(file_path.file_name().unwrap().to_str().unwrap());
            let metadata = extract_metadata(file_path.to_path_buf());

            if track_exists(&mut self.conn, &id) {
                self.update_track(id, file_path, metadata);
            } else {
                self.add_track(id, file_path, metadata);
            }
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

    fn update_track(&mut self, id: String, file_path: &Path, metadata: Metadata) {
        let mut update_track = UpdateTrackForm::default();
        let track: Track = tracks::table
            .filter(tracks::id.eq(&id))
            .get_result::<Track>(&mut self.conn)
            .unwrap();

        if let Some(artist) = metadata.artist {
            let artist_name_hash = sha256::digest(&artist);

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
                    }
                    Err(e) => {
                        error!("Failed to add artist to database!, {e}");
                    }
                };
                self.counter.artists += 1;
            }

            match track.artist_id {
                Some(artist_id) => {
                    if artist_id != artist_name_hash {
                        update_track.artist_id = Some(artist_name_hash.clone());
                    }
                }
                None => {
                    update_track.artist_id = Some(artist_name_hash.clone());
                }
            }

            if let Some(album) = metadata.album {
                if !album_exists(&mut self.conn, &album.to_string(), &artist_name_hash) {
                    let new_album = NewAlbum {
                        id: nanoid!(),
                        title: album.to_string(),
                        artist_id: artist_name_hash.clone(),
                    };

                    match diesel::insert_into(albums::table)
                        .values(&new_album)
                        .execute(&mut self.conn)
                    {
                        Ok(_) => {
                            info!("Added new album '{}' to database", new_album.title);
                        }
                        Err(e) => {
                            error!("Failed to add album to database!, {e}");
                        }
                    }

                    match track.album_id {
                        Some(album_id) => {
                            if album_id != new_album.id {
                                update_track.album_id = Some(new_album.id);
                            }
                        }
                        None => {
                            update_track.album_id = Some(new_album.id);
                        }
                    }

                    self.counter.albums += 1;
                }
            }
        }

        match diesel::delete(features::table.filter(features::track_id.eq(&id)))
            .execute(&mut self.conn)
        {
            Ok(_) => {
                for featured_artist in metadata.features {
                    let artist_name_hash = sha256::digest(&featured_artist);

                    if !artist_exists(&mut self.conn, &artist_name_hash) {
                        let new_artist = NewArtist {
                            id: artist_name_hash.clone(),
                            name: featured_artist.to_string(),
                        };

                        match diesel::insert_into(artists::table)
                            .values(new_artist)
                            .execute(&mut self.conn)
                        {
                            Ok(_) => {
                                info!("Added new artist '{}' to database", featured_artist);
                            }
                            Err(e) => {
                                error!("Failed to add artist to database!, {e}");
                            }
                        };

                        self.counter.artists += 1;
                    }

                    let new_feature = NewFeature {
                        id: nanoid!(),
                        artist_id: artist_name_hash,
                        track_id: id.clone(),
                    };

                    if let Err(e) = diesel::insert_into(features::table)
                        .values(&new_feature)
                        .execute(&mut self.conn)
                    {
                        error!("Failed to add featured artist to database!, {e}");
                    }
                }
            }
            Err(e) => {
                error!(
                    "Failed to update features for track('{}') from database!, {e}",
                    id
                );
            }
        }

        if track.title != metadata.title {
            update_track.title = Some(metadata.title);
        }

        if track.duration != metadata.duration {
            update_track.duration = Some(metadata.duration);
        }

        if let Some(year) = metadata.year {
            match track.year {
                Some(v) => {
                    if v != year {
                        update_track.year = Some(year);
                    }
                }
                None => {
                    update_track.year = Some(year);
                }
            }
        }

        if let Some(track_number) = metadata.track_number {
            match track.track_number {
                Some(v) => {
                    if v != track_number {
                        update_track.track_number = Some(track_number);
                    }
                }
                None => {
                    update_track.track_number = Some(track_number);
                }
            }
        }

        if track.path != file_path.display().to_string() {
            update_track.path = Some(file_path.display().to_string());
        }

        if diesel::update(tracks::table.filter(tracks::id.eq(&id)))
            .set::<UpdateTrackForm>(update_track)
            .execute(&mut self.conn)
            .is_ok()
        {
            info!("Updated track('{}') in database!", id);
        }
    }

    fn add_track(&mut self, id: String, file_path: &Path, metadata: Metadata) {
        let mut features_insert_queue: Vec<NewFeature> = Vec::new();

        let mut new_track = NewTrack {
            id,
            title: metadata.title,
            artist_id: None,
            album_id: None,
            duration: metadata.duration,
            year: metadata.year,
            track_number: metadata.track_number,
            path: file_path.display().to_string(),
        };

        if let Some(artist) = metadata.artist {
            let artist_name_hash = sha256::digest(&artist);

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

            if let Some(album) = metadata.album {
                if !album_exists(&mut self.conn, &album.to_string(), &artist_name_hash) {
                    let new_album = NewAlbum {
                        id: nanoid!(),
                        title: album.to_string(),
                        artist_id: artist_name_hash.clone(),
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

            new_track.artist_id = Some(artist_name_hash);
        }

        self.counter.tracks += 1;

        for feature_artist in metadata.features.into_iter() {
            let feature_artist_name_hash = sha256::digest(&feature_artist);

            // Create artist if does not exists
            if !artist_exists(&mut self.conn, &feature_artist_name_hash) {
                let new_artist = NewArtist {
                    id: feature_artist_name_hash.clone(),
                    name: feature_artist.to_string(),
                };

                match diesel::insert_into(artists::table)
                    .values(new_artist)
                    .execute(&mut self.conn)
                {
                    Ok(_) => {
                        info!("Added new artist '{}' to database", &feature_artist);
                        self.counter.artists += 1;
                    }
                    Err(e) => {
                        error!("Failed to add artist to database!, {e}");
                    }
                };
            }

            if !feature_exists(
                &mut self.conn,
                feature_artist.to_string(),
                new_track.id.clone(),
            ) {
                let new_feature = NewFeature {
                    id: nanoid!(),
                    artist_id: feature_artist_name_hash.clone(),
                    track_id: new_track.id.clone(),
                };

                features_insert_queue.push(new_feature);
            }
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
                        "Added new {} featured artists on '{}' to database",
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
