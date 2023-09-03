use crate::db;
use crate::models::scan_info::ScanInfo;
use crate::schema;
use diesel::prelude::*;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Stats {
    artists: i64,
    albums: i64,
    tracks: i64,
    last_scan: Option<ScanInfo>,
}

#[get("/stats")]
pub fn stats() -> Json<Stats> {
    let mut conn = db::establish_connection();

    let scans = schema::scan_info::table
        .order(schema::scan_info::id)
        .load::<ScanInfo>(&mut conn)
        .unwrap();

    let artists = schema::artists::table
        .count()
        .get_result::<i64>(&mut conn)
        .unwrap_or(0);

    let albums = schema::albums::table
        .count()
        .get_result::<i64>(&mut conn)
        .unwrap_or(0);

    let tracks = schema::tracks::table
        .count()
        .get_result::<i64>(&mut conn)
        .unwrap_or(0);

    let mut stats = Stats {
        artists,
        albums,
        tracks,
        last_scan: None,
    };

    if let Some(last_scan) = scans.last() {
        stats.last_scan = Some(*last_scan);
    }

    Json(stats)
}
