use super::Response;
use crate::db;
use crate::models::albums::Album;
use crate::schema;
use diesel::prelude::*;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Data {
    albums: Vec<Album>,
    offset: Option<i64>,
    limit: Option<i64>,
    total: i64,
}

#[derive(Serialize, Deserialize)]
pub struct AlbumData {
    album: Album,
}

#[get("/albums?<artist>&<offset>&<limit>")]
pub fn albums(
    artist: Option<String>,
    offset: Option<i64>,
    limit: Option<i64>,
) -> Json<Response<Data>> {
    let mut conn = db::establish_connection();
    let mut query = schema::albums::table.into_boxed();

    if let Some(artist) = artist {
        query = query.filter(schema::albums::artist_id.eq(artist));
    }

    if let Some(offset) = offset {
        query = query.offset(offset);
    }

    if let Some(limit) = limit {
        query = query.limit(limit);
    }

    let albums = match query.load::<Album>(&mut conn) {
        Ok(v) => v,
        Err(e) => return Json(Response::error { msg: e.to_string() }),
    };

    let total = schema::albums::table
        .count()
        .get_result::<i64>(&mut conn)
        .unwrap();

    Json(Response::data(Data {
        albums,
        offset,
        limit,
        total,
    }))
}

#[get("/albums/<id>")]
pub fn album(id: String) -> Json<Response<AlbumData>> {
    let mut conn = db::establish_connection();

    let album = match schema::albums::table
        .filter(schema::albums::id.eq(id))
        .get_result::<Album>(&mut conn)
    {
        Ok(v) => v,
        Err(e) => return Json(Response::error { msg: e.to_string() }),
    };

    Json(Response::data(AlbumData { album }))
}
