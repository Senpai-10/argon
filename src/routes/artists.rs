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
    offset: Option<i64>,
    limit: Option<i64>,
    total: i64,
}

#[get("/artists?<offset>&<limit>")]
pub fn artists(offset: Option<i64>, limit: Option<i64>) -> Json<Response<Data>> {
    let mut conn = db::establish_connection();
    let mut query = schema::artists::table.into_boxed();

    if let Some(offset) = offset {
        query = query.offset(offset);
    }

    if let Some(limit) = limit {
        query = query.limit(limit);
    }

    let artists = match query.load::<Artist>(&mut conn) {
        Ok(v) => v,
        Err(e) => return Json(Response::error { msg: e.to_string() }),
    };

    let total = schema::artists::table
        .count()
        .get_result::<i64>(&mut conn)
        .unwrap();

    Json(Response::data(Data {
        artists,
        offset,
        limit,
        total,
    }))
}
