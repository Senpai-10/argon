use crate::db;
use crate::models::tokens::Token;
use crate::models::users::User;
use crate::routes::ResError;
use crate::schema;
use chrono::Utc;
use diesel::prelude::*;
use diesel::PgConnection;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};

#[derive(Debug)]
pub struct Authorization<'r> {
    pub user: User,
    pub token: &'r str,
}

#[derive(Debug)]
pub enum AuthorizationError {
    Missing,
    Invalid,
}

impl AuthorizationError {
    pub fn res_error(&self) -> ResError {
        match self {
            Self::Missing => ResError {
                msg: "".into(),
                detail: "Missing authentication token".into(),
            },
            Self::Invalid => ResError {
                msg: "".into(),
                detail: "Invaild authentication token".into(),
            },
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Authorization<'r> {
    type Error = AuthorizationError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match req.headers().get_one("Authorization") {
            None => Outcome::Failure((Status::BadRequest, AuthorizationError::Missing)),
            Some(key) => {
                if !key.contains("Bearer ") {
                    return Outcome::Failure((Status::BadRequest, AuthorizationError::Invalid));
                }

                let token: &str = key.split_whitespace().collect::<Vec<_>>()[1];

                let mut conn = db::establish_connection();

                if !validate_token(&mut conn, token) {
                    return Outcome::Failure((Status::BadRequest, AuthorizationError::Invalid));
                }

                let user: User = schema::tokens::table
                    .filter(schema::tokens::id.eq(&token))
                    .inner_join(schema::users::table)
                    .select(User::as_select())
                    .get_result::<User>(&mut conn)
                    .unwrap();

                Outcome::Success(Authorization { user, token })
            }
        }
    }
}

pub fn validate_token(conn: &mut PgConnection, token_id: &str) -> bool {
    let q: Result<Token, _> = schema::tokens::table
        .filter(schema::tokens::id.eq(token_id))
        .get_result::<Token>(conn);

    if let Ok(token) = q {
        // Check if the token has expired
        let now = Utc::now().naive_local();

        if now > token.expires_at {
            return false;
            // return Err(Json(ResError {
            //     msg: "Failed to validate token".to_string(),
            //     detail: "Token expired!".into(),
            // }));
        }

        return true;
    };

    // Err(Json(ResError {
    //     msg: "Failed to validate token".to_string(),
    //     detail: "Invaild token id".into(),
    // }))

    false
}
