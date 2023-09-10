use crate::models::tracks::Track;
use crate::models::users::User;
use crate::schema;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Queryable, Selectable, Identifiable, Associations, Debug, Serialize, Deserialize, Clone,
)]
#[diesel(table_name = schema::favorites)]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(belongs_to(Track, foreign_key = track_id))]
pub struct Favorite {
    pub id: String,
    pub user_id: String,
    pub track_id: String,

    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = schema::favorites)]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(belongs_to(Track, foreign_key = track_id))]
pub struct NewFavorite {
    pub id: String,
    pub user_id: String,
    pub track_id: String,
}
