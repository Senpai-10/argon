use crate::models::albums::Album;
use crate::models::artists::Artist;
use crate::schema;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Debug, Serialize, Deserialize)]
#[diesel(table_name = schema::songs)]
#[diesel(belongs_to(Artist, foreign_key = artist_name))]
#[diesel(belongs_to(Album, foreign_key = album_title))]
pub struct Song {
    pub id: String,
    pub title: String,
    pub artist_name: String,
    pub album_title: String,
    pub length: i32,
    pub year: Option<i32>,
    pub track: Option<i32>,
    pub path: String,
}
