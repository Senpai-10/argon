use super::{create_session, Data, ResError, Response, UserCreds};
use crate::db;
use crate::models::users::{NewUser, User};
use crate::schema;
use diesel::dsl::{exists, select};
use diesel::prelude::*;
use nanoid::nanoid;
use pwhash::bcrypt;
use rocket::form::Form;
use rocket::serde::json::Json;

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
