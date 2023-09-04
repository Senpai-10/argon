use crate::models::artists::Artist;
use crate::schema;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, Serialize, Deserialize)]
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
