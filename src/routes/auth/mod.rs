mod login;
mod logout;
mod signup;

use crate::models::tokens::NewToken;
use crate::schema;
use chrono::Utc;
use diesel::prelude::*;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Data {
    token: String,
}

#[derive(Deserialize, Serialize, FromForm)]
pub struct UserCreds {
    pub username: String,
    pub password: String,
}

pub fn create_token(conn: &mut PgConnection, user_id: String) -> String {
    let expires_at = Utc::now()
        .naive_utc()
        .checked_add_months(chrono::Months::new(3))
        .unwrap();

    let new_token = NewToken {
        id: nanoid!(128),
        user_id,
        expires_at,
    };

    _ = diesel::insert_into(schema::tokens::table)
        .values(&new_token)
        .execute(conn);

    new_token.id
}

pub fn routes() -> Vec<rocket::Route> {
    routes![login::rt, logout::rt, signup::rt]
}
