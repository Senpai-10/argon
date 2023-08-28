use crate::models::artists::Artist;
use crate::models::songs::Song;
use crate::schema;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Debug, Serialize, Deserialize)]
#[diesel(table_name = schema::features)]
#[diesel(belongs_to(Artist, foreign_key = artist_name))]
#[diesel(belongs_to(Song, foreign_key = song_id))]
pub struct Feature {
    pub id: String,
    pub artist_name: String,
    pub song_id: String,
}
