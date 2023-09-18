use id3::TagLike;
use mpeg_audio_header::{Header, ParseMode};
use std::path::PathBuf;

pub struct Metadata {
    pub title: String,
    pub artist: Option<String>,
    pub features: Vec<String>,
    pub album: Option<String>,
    pub duration: i32,
    pub year: Option<i32>,
    pub track_number: Option<i32>,
}

impl Default for Metadata {
    fn default() -> Self {
        Self {
            title: String::from("Untitled"),
            artist: None,
            features: Vec::new(),
            album: None,
            duration: 0,
            year: None,
            track_number: None,
        }
    }
}

pub fn extract_metadata(file_path: PathBuf) -> Metadata {
    let mut metadata = Metadata::default();
    let tag = id3::Tag::read_from_path(&file_path).unwrap();

    if let Some(title) = tag.title() {
        metadata.title = title.to_string()
    }

    if let Some(artists) = tag.artists() {
        for (index, artist) in artists.into_iter().enumerate() {
            // Main artist on the track
            if index == 0 {
                metadata.artist = Some(artist.to_string());
                continue;
            }

            metadata.features.push(artist.to_string());
        }
    }

    if let Some(album) = tag.album() {
        metadata.album = Some(album.to_string());
    }

    if let Ok(header) = Header::read_from_path(&file_path, ParseMode::PreferVbrHeaders) {
        metadata.duration = header.total_duration.as_secs() as i32
    }

    if let Some(year) = tag.year() {
        metadata.year = Some(year)
    }

    if let Some(track_number) = tag.track() {
        metadata.track_number = Some(track_number as i32)
    }

    metadata
}
