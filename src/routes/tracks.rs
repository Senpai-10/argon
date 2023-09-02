use super::Response;
use crate::db;
use crate::models::tracks::Track;
use crate::schema;
use diesel::prelude::*;
use id3::frame::PictureType;
use rocket::response::status::NotFound;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct TracksData {
    pub tracks: Vec<Track>,
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub total: i64,
}

#[derive(Serialize, Deserialize)]
pub struct TrackData {
    pub track: Track,
}

#[get("/tracks?<offset>&<limit>")]
pub fn tracks(offset: Option<i64>, limit: Option<i64>) -> Json<Response<TracksData>> {
    let mut conn = db::establish_connection();
    let mut query = schema::tracks::dsl::tracks.into_boxed();

    if let Some(offset) = offset {
        query = query.offset(offset);
    }

    if let Some(limit) = limit {
        query = query.limit(limit);
    }

    let tracks = match query.load::<Track>(&mut conn) {
        Ok(v) => v,
        Err(e) => return Json(Response::error { msg: e.to_string() }),
    };

    let total_tracks = schema::tracks::dsl::tracks
        .count()
        .get_result::<i64>(&mut conn)
        .unwrap();

    Json(Response::data(TracksData {
        tracks,
        offset,
        limit,
        total: total_tracks,
    }))
}

#[get("/tracks/<id>")]
pub fn track(id: String) -> Json<Response<TrackData>> {
    let mut conn = db::establish_connection();

    let track = match schema::tracks::dsl::tracks
        .filter(schema::tracks::id.eq(&id))
        .get_result::<Track>(&mut conn)
    {
        Ok(v) => v,
        Err(e) => return Json(Response::error { msg: e.to_string() }),
    };

    Json(Response::data(TrackData { track }))
}

#[get("/tracks/<id>/cover")]
pub fn track_cover(id: String) -> Result<Vec<u8>, NotFound<String>> {
    let mut conn = db::establish_connection();

    let track = match schema::tracks::dsl::tracks
        .filter(schema::tracks::id.eq(&id))
        .get_result::<Track>(&mut conn)
    {
        Ok(v) => v,
        Err(_) => return Err(NotFound("Track not found!".to_string())),
    };

    let track_file = Path::new(&track.path);

    if !track_file.exists() {
        return Err(NotFound("Track file not found!".to_string()));
    }

    let tag = id3::Tag::read_from_path(track_file).unwrap();

    for pic in tag.pictures() {
        if pic.picture_type == PictureType::CoverFront {
            return Ok(pic.data.to_vec());
        }
    }

    Err(NotFound("Cover not found!".to_string()))
}
