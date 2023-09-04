use crate::models::albums::Album;
use crate::models::tracks::Track;
use crate::schema;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, Serialize, Deserialize)]
#[diesel(table_name = schema::albums_tracks)]
#[diesel(belongs_to(Album, foreign_key = album_id))]
#[diesel(belongs_to(Track, foreign_key = track_id))]
pub struct AlbumTrack {
    pub id: String,
    pub album_id: String,
    pub track_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = schema::albums_tracks)]
#[diesel(belongs_to(Album, foreign_key = album_id))]
#[diesel(belongs_to(Track, foreign_key = track_id))]
pub struct NewAlbumTrack {
    pub id: String,
    pub album_id: String,
    pub track_id: String,
}
