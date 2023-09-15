use crate::models::tracks::TrackInRes;
use crate::models::users::User;
use crate::schema;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct PlaylistInRes {
    #[serde(flatten)]
    pub playlist: Playlist,

    pub tracks: Vec<TrackInRes>,
}

#[derive(
    Queryable, Selectable, Identifiable, Associations, Debug, Serialize, Deserialize, Clone,
)]
#[diesel(table_name = schema::playlists)]
#[diesel(belongs_to(User, foreign_key = user_id))]
pub struct Playlist {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub description: Option<String>,
    pub is_public: bool,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = schema::playlists)]
#[diesel(belongs_to(User, foreign_key = user_id))]
pub struct NewPlaylist {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub description: Option<String>,
    pub is_public: bool,
}
