use super::Data;
use crate::routes::prelude::*;

#[post("/logout")]
pub fn rt(auth: Authorization) -> Json<Response<Data>> {
    let mut conn = establish_connection();

    match diesel::delete(sessions::table.filter(sessions::id.eq(&auth.session_id)))
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
