use crate::schema;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = schema::covers)]
#[diesel(belongs_to(Track, foreign_key = track_id))]
pub struct Cover {
    pub track_id: String,
    pub image_data: Vec<u8>,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = schema::covers)]
#[diesel(belongs_to(Track, foreign_key = track_id))]
pub struct NewCover {
    pub track_id: String,
    pub image_data: Vec<u8>,
}
