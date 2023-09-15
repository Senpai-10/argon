use crate::models::playlists::Playlist;
use crate::models::tracks::Track;
use crate::schema;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Queryable, Selectable, Identifiable, Associations, Debug, Serialize, Deserialize, Clone,
)]
#[diesel(table_name = schema::playlists_tracks)]
#[diesel(belongs_to(Playlist, foreign_key = playlist_id))]
#[diesel(belongs_to(Track, foreign_key = track_id))]
pub struct PlaylistTrack {
    pub id: String,
    pub playlist_id: String,
    pub track_id: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = schema::playlists_tracks)]
#[diesel(belongs_to(Playlist, foreign_key = playlist_id))]
#[diesel(belongs_to(Track, foreign_key = track_id))]
pub struct NewPlaylistTrack {
    pub id: String,
    pub playlist_id: String,
    pub track_id: String,
}
