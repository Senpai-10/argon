use super::{create_token, Data, UserCreds};
use crate::models::users::{NewUser, User};
use crate::routes::prelude::*;
use diesel::dsl::{exists, select};
use nanoid::nanoid;
use pwhash::bcrypt;
use rocket::form::Form;

#[post("/signup", data = "<user_creds>")]
pub fn rt(user_creds: Form<UserCreds>) -> Json<Response<Data>> {
    let mut conn = establish_connection();

    if select(exists(
        users::table.filter(users::name.eq(&user_creds.username)),
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

    match diesel::insert_into(users::table)
        .values(new_user)
        .get_result::<User>(&mut conn)
    {
        Ok(user) => {
            let token_id = create_token(&mut conn, user.id);

            Json(Response::data(Data { token: token_id }))
        }
        Err(e) => Json(Response::error(ResError {
            msg: e.to_string(),
            detail: "Failed to create user".into(),
        })),
    }
}
