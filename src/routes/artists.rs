use super::Response;
use crate::db;
use crate::models::artists::Artist;
use crate::schema;
use diesel::prelude::*;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Data {
    artists: Vec<Artist>,
}

#[get("/artists")]
pub fn artists() -> Json<Response<Data>> {
    let mut conn = db::establish_connection();

    let artists = match schema::artists::dsl::artists.load::<Artist>(&mut conn) {
        Ok(v) => v,
        Err(e) => return Json(Response::error { msg: e.to_string() }),
    };

    Json(Response::data(Data { artists }))
}
