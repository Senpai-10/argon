use crate::models::users::User;
use crate::schema;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Queryable, Selectable, Identifiable, Associations, Debug, Serialize, Deserialize, Clone,
)]
#[diesel(table_name = schema::sessions)]
#[diesel(belongs_to(User, foreign_key = user_id))]
pub struct Session {
    pub id: String,
    pub user_id: String,

    pub created_at: NaiveDateTime,
    pub expires_at: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = schema::sessions)]
#[diesel(belongs_to(User, foreign_key = user_id))]
pub struct NewSession {
    pub id: String,
    pub user_id: String,

    pub expires_at: NaiveDateTime,
}
