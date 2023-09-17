mod login;
mod logout;
mod signup;

use super::{ResError, Response};

use crate::models::sessions::NewSession;
use crate::schema;
use chrono::Utc;
use diesel::prelude::*;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Data {
    session_id: String,
}

#[derive(Deserialize, Serialize, FromForm)]
pub struct UserCreds {
    pub username: String,
    pub password: String,
}

pub fn create_session(conn: &mut PgConnection, user_id: String) -> String {
    let expires_at = Utc::now()
        .naive_utc()
        .checked_add_months(chrono::Months::new(3))
        .unwrap();

    let new_session = NewSession {
        id: nanoid!(128),
        user_id,
        expires_at,
    };

    _ = diesel::insert_into(schema::sessions::table)
        .values(&new_session)
        .execute(conn);

    new_session.id
}

pub fn routes() -> Vec<rocket::Route> {
    routes![login::rt, logout::rt, signup::rt]
}
