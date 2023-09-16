use super::{Data, ResError, Response};
use crate::auth::Authorization;
use crate::db;
use crate::schema;
use diesel::prelude::*;
use rocket::serde::json::Json;

#[post("/logout")]
pub fn logout(auth: Authorization) -> Json<Response<Data>> {
    let mut conn = db::establish_connection();

    match diesel::delete(schema::sessions::table.filter(schema::sessions::id.eq(&auth.session_id)))
        .execute(&mut conn)
    {
        Ok(_) => Json(Response::data(Data {
            session_id: auth.session_id.to_string(),
        })),
        Err(_) => Json(Response::error(ResError {
            msg: "Failed to logout".into(),
            detail: "Failed to remove session id".into(),
        })),
    }
}
