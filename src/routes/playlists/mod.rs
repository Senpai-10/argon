mod all_playlists;
mod new_playlist;
mod new_playlist_track;
mod one_playlist;
mod remove_playlist;
mod remove_playlist_track;
mod update_playlist;

use super::{ResError, Response};
use crate::models::playlists::Playlist;
use crate::models::tracks::TrackInRes;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct PlaylistData {
    playlist: Playlist,
}

#[derive(Deserialize, Serialize)]
pub struct TrackData {
    track: TrackInRes,
}

pub fn routes() -> Vec<rocket::Route> {
    routes![
        all_playlists::all_playlists,
        one_playlist::one_playlist,
        new_playlist::new_playlist,
        remove_playlist::remove_playlist,
        new_playlist_track::new_playlist_track,
        update_playlist::update_playlist,
        remove_playlist_track::remove_playlist_track,
    ]
}
