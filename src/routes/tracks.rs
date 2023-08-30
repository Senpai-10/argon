use crate::db;
use crate::models::tracks::Track;
use crate::schema;
use diesel::prelude::*;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ResTracks {
    pub tracks: Vec<Track>,
    pub total: i64,
}

#[derive(Serialize, Deserialize)]
pub struct ResTracksFeed {
    pub tracks: Vec<Track>,
    pub offset: i64,
    pub limit: i64,
    pub total: i64,
}

#[get("/tracks")]
pub fn tracks() -> Json<ResTracks> {
    let mut conn = db::establish_connection();
    let tracks = schema::tracks::dsl::tracks
        .load::<Track>(&mut conn)
        .expect("Failed to fetch tracks");

    let total_tracks = schema::tracks::dsl::tracks
        .count()
        .get_result::<i64>(&mut conn)
        .unwrap();

    Json(ResTracks {
        tracks,
        total: total_tracks,
    })
}

#[get("/tracks/feed?<offset>&<limit>")]
pub fn tracks_feed(offset: i64, limit: i64) -> Json<ResTracksFeed> {
    let mut conn = db::establish_connection();
    let tracks = schema::tracks::dsl::tracks
        .offset(offset)
        .limit(limit)
        .load::<Track>(&mut conn)
        .expect("Failed to fetch tracks");

    let total_tracks = schema::tracks::dsl::tracks
        .count()
        .get_result::<i64>(&mut conn)
        .unwrap();

    Json(ResTracksFeed {
        tracks,
        offset,
        limit,
        total: total_tracks,
    })
}
