use super::Response;
use crate::db;
use crate::models::tracks::Track;
use crate::schema;
use diesel::prelude::*;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

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
    let mut query = schema::tracks::table.into_boxed();

    if let Some(offset) = offset {
        query = query.offset(offset);
    }

    if let Some(limit) = limit {
        query = query.limit(limit);
    }

    let tracks = match query.load::<Track>(&mut conn) {
        Ok(v) => v,
        Err(e) => {
            return Json(Response::error {
                msg: e.to_string(),
                detail: String::from("Failed to get tracks"),
            })
        }
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
        Err(e) => {
            return Json(Response::error {
                msg: e.to_string(),
                detail: format!("Track '{id}' does not exists!"),
            })
        }
    };

    Json(Response::data(TrackData { track }))
}
