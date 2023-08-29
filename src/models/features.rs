use crate::models::artists::Artist;
use crate::models::tracks::Track;
use crate::schema;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Debug, Serialize, Deserialize)]
#[diesel(table_name = schema::features)]
#[diesel(belongs_to(Artist, foreign_key = artist_name))]
#[diesel(belongs_to(Track, foreign_key = track_id))]
pub struct Feature {
    pub id: String,
    pub artist_name: String,
    pub track_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}
