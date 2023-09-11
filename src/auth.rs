use crate::db;
use crate::models::sessions::Session;
use crate::models::users::User;
use crate::schema;
use chrono::Utc;
use diesel::prelude::*;
use diesel::PgConnection;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};

#[derive(Debug)]
pub struct Authorization<'r> {
    pub user: User,
    pub session_id: &'r str,
}

#[derive(Debug)]
pub enum AuthorizationError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Authorization<'r> {
    type Error = AuthorizationError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        /// Returns true if `key` is a valid API key string.
        fn is_valid(session_id: &str) -> bool {
            let mut conn = db::establish_connection();
            validate_session(&mut conn, session_id)
        }

        match req.headers().get_one("Authorization") {
            None => Outcome::Failure((Status::BadRequest, AuthorizationError::Missing)),
            Some(key) if is_valid(key) => {
                let mut conn = db::establish_connection();

                let user: User = schema::sessions::table
                    .filter(schema::sessions::id.eq(&key))
                    .inner_join(schema::users::table)
                    .select(User::as_select())
                    .get_result::<User>(&mut conn)
                    .unwrap();

                Outcome::Success(Authorization {
                    user,
                    session_id: key,
                })
            }
            Some(_) => Outcome::Failure((Status::BadRequest, AuthorizationError::Invalid)),
        }
    }
}

pub fn validate_session(conn: &mut PgConnection, session_id: &str) -> bool {
    let q: Result<Session, _> = schema::sessions::table
        .filter(schema::sessions::id.eq(session_id))
        .get_result::<Session>(conn);

    if let Ok(session) = q {
        // Check if the session expired
        let now = Utc::now().naive_local();

        if now > session.expires_at {
            return false;
            // return Err(Json(ResError {
            //     msg: "Failed to validate session".to_string(),
            //     detail: "Session expired!".into(),
            // }));
        }

        return true;
    };

    // Err(Json(ResError {
    //     msg: "Failed to validate session".to_string(),
    //     detail: "Invaild session id".into(),
    // }))

    false
}
