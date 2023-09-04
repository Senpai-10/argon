use crate::schema;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Identifiable, Queryable, Selectable, Debug, Serialize, Deserialize)]
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
