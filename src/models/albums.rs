use crate::schema;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = schema::albums)]
pub struct Album {
    pub title: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = schema::albums)]
pub struct NewAlbum {
    pub title: String,
}
