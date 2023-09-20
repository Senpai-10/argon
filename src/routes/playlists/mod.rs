mod all_playlists;
mod new_playlist;
mod new_playlist_track;
mod one_playlist;
mod remove_playlist;
mod remove_playlist_track;
mod update_playlist;

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
        all_playlists::rt,
        one_playlist::rt,
        new_playlist::rt,
        remove_playlist::rt,
        new_playlist_track::rt,
        update_playlist::rt,
        remove_playlist_track::rt,
    ]
}
