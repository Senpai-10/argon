use crate::models::albums::Album;
use crate::models::artists::Artist;
use crate::schema;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct TrackInRes {
    #[serde(flatten)]
    pub track: Track,

    pub artist: Option<Artist>,
    pub album: Option<Album>,
    pub features: Vec<Artist>,
}

#[derive(Identifiable, Queryable, Selectable, Associations, Debug, Serialize, Deserialize)]
#[diesel(table_name = schema::tracks)]
#[diesel(belongs_to(Artist, foreign_key = artist_id))]
#[diesel(belongs_to(Album, foreign_key = album_id))]
pub struct Track {
    pub id: String,
    pub title: String,
    pub artist_id: Option<String>,
    pub album_id: Option<String>,
    pub duration: i32,
    pub year: Option<i32>,
    pub track_number: Option<i32>,
    pub last_play: Option<NaiveDateTime>,
    pub plays: i32,
    pub path: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = schema::tracks)]
#[diesel(belongs_to(Artist, foreign_key = artist_id))]
#[diesel(belongs_to(Album, foreign_key = album_id))]
pub struct NewTrack {
    pub id: String,
    pub title: String,
    pub artist_id: Option<String>,
    pub album_id: Option<String>,
    pub duration: i32,
    pub year: Option<i32>,
    pub track_number: Option<i32>,
    pub path: String,
}
