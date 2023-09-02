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
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub total: i64,
}

#[get("/tracks?<offset>&<limit>")]
pub fn tracks(offset: Option<i64>, limit: Option<i64>) -> Json<ResTracksFeed> {
    let mut conn = db::establish_connection();
    let mut query = schema::tracks::dsl::tracks.into_boxed();

    if let Some(offset) = offset {
        query = query.offset(offset);
    }

    if let Some(limit) = limit {
        query = query.limit(limit);
    }

    let tracks = query
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
