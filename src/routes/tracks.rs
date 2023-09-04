use super::Response;
use crate::db;
use crate::models::tracks::Track;
use crate::schema;
use diesel::prelude::*;
use id3::frame::PictureType;
use rocket::response::status::NotFound;
use rocket::serde::json::Json;
use rocket_seek_stream::SeekStream;
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

#[get("/tracks?<artist>&<album>&<offset>&<limit>")]
pub fn tracks(
    artist: Option<String>,
    album: Option<String>,
    offset: Option<i64>,
    limit: Option<i64>,
) -> Json<Response<TracksData>> {
    let mut conn = db::establish_connection();
    let mut query = schema::tracks::table.into_boxed();

    if let Some(artist) = artist {
        query = query.filter(schema::tracks::artist_id.eq(artist))
    }

    if let Some(album) = album {
        query = query.filter(schema::tracks::album_id.eq(album))
    }

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

    let total_tracks = schema::tracks::table
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

    let track = match schema::tracks::table
        .filter(schema::tracks::id.eq(&id))
        .get_result::<Track>(&mut conn)
    {
        Ok(v) => v,
        Err(e) => return Json(Response::error { msg: e.to_string() }),
    };

    Json(Response::data(TrackData { track }))
}

#[get("/tracks/<id>/stream")]
pub fn track_stream<'a>(id: String) -> Result<SeekStream<'a>, NotFound<String>> {
    let mut conn = db::establish_connection();

    let track: Track = match schema::tracks::table
        .filter(schema::tracks::id.eq(&id))
        .get_result::<Track>(&mut conn)
    {
        Ok(v) => v,
        Err(e) => return Err(NotFound(e.to_string())),
    };

    // Update track global plays
    if let Err(e) = diesel::update(schema::tracks::table)
        .filter(schema::tracks::id.eq(&id))
        .set(schema::tracks::plays.eq(track.plays + 1))
        .execute(&mut conn)
    {
        error!("Failed to update track plays!, {e}")
    }

    let file = Path::new(&track.path);

    if file.exists() {
        return match SeekStream::from_path(file) {
            Ok(s) => Ok(s),
            Err(e) => Err(NotFound(e.to_string())),
        };
    }

    Err(NotFound("Track file not found!".into()))
}

#[get("/tracks/<id>/cover")]
pub fn track_cover(id: String) -> Result<Vec<u8>, NotFound<String>> {
    let mut conn = db::establish_connection();

    let track = match schema::tracks::table
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
