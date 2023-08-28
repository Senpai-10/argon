use crate::schema;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = schema::artists)]
pub struct Artist {
    pub name: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = schema::artists)]
pub struct NewArtist {
    pub name: String,
}
