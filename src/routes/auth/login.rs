use super::{create_token, Data, UserCreds};
use crate::models::users::User;
use crate::routes::prelude::*;
use diesel::dsl::{exists, select};
use pwhash::bcrypt;
use rocket::form::Form;

#[post("/login", data = "<user_creds>")]
pub fn rt(user_creds: Form<UserCreds>) -> Json<Response<Data>> {
    let mut conn = establish_connection();

    if !select(exists(
        users::table.filter(users::name.eq(&user_creds.username)),
    ))
    .get_result::<bool>(&mut conn)
    .unwrap()
    {
        return Json(Response::error(ResError {
            msg: "Failed to login".into(),
            detail: "User does not exists".into(),
        }));
    };

    if let Ok(user) = users::table
        .filter(users::name.eq(&user_creds.username))
        .get_result::<User>(&mut conn)
    {
        if !bcrypt::verify(user_creds.password.clone(), &user.password) {
            return Json(Response::error(ResError {
                msg: "Failed to login".into(),
                detail: "Incorrect password".into(),
            }));
        }

        let token_id = create_token(&mut conn, user.id);

        return Json(Response::data(Data { token: token_id }));
    }

    Json(Response::error(ResError {
        msg: "Failed to login".into(),
        detail: "user does not exists".into(),
    }))
}
