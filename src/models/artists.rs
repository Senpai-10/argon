use crate::models::tracks::TrackInRes;
use crate::schema;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct ArtistWithTracks {
    #[serde(flatten)]
    pub artist: Artist,
    pub featured_on: Vec<TrackInRes>,
    pub tracks: Vec<TrackInRes>,
}

#[derive(Identifiable, Queryable, Selectable, Debug, Serialize, Deserialize, Clone)]
#[diesel(table_name = schema::artists)]
pub struct Artist {
    pub id: String,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = schema::artists)]
pub struct NewArtist {
    pub id: String,
    pub name: String,
}
