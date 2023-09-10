use super::{ResError, Response};
use crate::db;
use crate::models::sessions::NewSession;
use crate::models::users::{NewUser, User};
use crate::schema;
use chrono::Utc;
use diesel::dsl::{exists, select};
use diesel::prelude::*;
use nanoid::nanoid;
use pwhash::bcrypt;
use rocket::form::Form;
use rocket::serde::json::Json;
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

fn create_session(conn: &mut PgConnection, user_id: String) -> String {
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

#[post("/signup", data = "<user_creds>")]
pub fn signup(user_creds: Form<UserCreds>) -> Json<Response<Data>> {
    let mut conn = db::establish_connection();

    if select(exists(
        schema::users::table.filter(schema::users::name.eq(&user_creds.username)),
    ))
    .get_result::<bool>(&mut conn)
    .unwrap()
    {
        return Json(Response::error(ResError {
            msg: "Failed to signup".into(),
            detail: "Username already exists".into(),
        }));
    };

    let hash = bcrypt::hash(user_creds.password.clone()).unwrap();

    let new_user = NewUser {
        id: nanoid!(),
        name: user_creds.username.clone(),
        password: hash,
    };

    match diesel::insert_into(schema::users::table)
        .values(new_user)
        .get_result::<User>(&mut conn)
    {
        Ok(user) => {
            let session_id = create_session(&mut conn, user.id);

            Json(Response::data(Data { session_id }))
        }
        Err(e) => Json(Response::error(ResError {
            msg: e.to_string(),
            detail: "Failed to create user".into(),
        })),
    }
}

#[post("/login", data = "<user_creds>")]
pub fn login(user_creds: Form<UserCreds>) -> Json<Response<Data>> {
    let mut conn = db::establish_connection();

    if !select(exists(
        schema::users::table.filter(schema::users::name.eq(&user_creds.username)),
    ))
    .get_result::<bool>(&mut conn)
    .unwrap()
    {
        return Json(Response::error(ResError {
            msg: "Failed to login".into(),
            detail: "User does not exists".into(),
        }));
    };

    if let Ok(user) = schema::users::table
        .filter(schema::users::name.eq(&user_creds.username))
        .get_result::<User>(&mut conn)
    {
        if !bcrypt::verify(user_creds.password.clone(), &user.password) {
            return Json(Response::error(ResError {
                msg: "Failed to login".into(),
                detail: "Incorrect password".into(),
            }));
        }

        let session_id = create_session(&mut conn, user.id);

        return Json(Response::data(Data { session_id }));
    }

    Json(Response::error(ResError {
        msg: "Failed to login".into(),
        detail: "user does not exists".into(),
    }))
}
