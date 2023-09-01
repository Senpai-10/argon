use crate::schema;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Debug, Serialize, Deserialize)]
#[diesel(table_name = schema::features)]
#[diesel(belongs_to(Artist, foreign_key = artist_id))]
#[diesel(belongs_to(Track, foreign_key = track_id))]
pub struct Feature {
    pub id: String,
    pub artist_id: String,
    pub track_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = schema::features)]
#[diesel(belongs_to(Artist, foreign_key = artist_id))]
#[diesel(belongs_to(Track, foreign_key = track_id))]
pub struct NewFeature {
    pub id: String,
    pub artist_id: String,
    pub track_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}
