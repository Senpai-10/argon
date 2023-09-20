use super::Data;
use crate::routes::prelude::*;

#[post("/logout")]
pub fn rt(auth: Authorization) -> Json<Response<Data>> {
    let mut conn = establish_connection();

    match diesel::delete(tokens::table.filter(tokens::id.eq(&auth.token))).execute(&mut conn) {
        Ok(_) => Json(Response::data(Data {
            token: auth.token.to_string(),
        })),
        Err(_) => Json(Response::error(ResError {
            msg: "Failed to logout".into(),
            detail: "Failed to remove session id".into(),
        })),
    }
}
