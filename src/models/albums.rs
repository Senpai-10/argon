use crate::models::artists::Artist;
use crate::models::tracks::TrackInRes;
use crate::schema;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct AlbumWithTracks {
    #[serde(flatten)]
    pub album: Album,
    pub artist: Artist,
    pub tracks: Vec<TrackInRes>,
}

#[derive(
    Queryable, Selectable, Identifiable, Associations, Debug, Serialize, Deserialize, Clone,
)]
#[diesel(table_name = schema::albums)]
#[diesel(belongs_to(Artist, foreign_key = artist_id))]
pub struct Album {
    pub id: String,
    pub title: String,
    pub artist_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = schema::albums)]
#[diesel(belongs_to(Artist, foreign_key = artist_id))]
pub struct NewAlbum {
    pub id: String,
    pub title: String,
    pub artist_id: String,
}
