use crate::schema;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Identifiable, Queryable, Selectable, Debug, Serialize, Deserialize, Clone)]
#[diesel(table_name = schema::users)]
pub struct User {
    pub id: String,
    pub name: String,
    pub password: String,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = schema::users)]
pub struct NewUser {
    pub id: String,
    pub name: String,
    pub password: String,
}
