use crate::models::scan_info::ScanInfo;
use crate::routes::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct Stats {
    artists: i64,
    albums: i64,
    tracks: i64,
    last_scan: Option<ScanInfo>,
}

#[get("/stats")]
pub fn rt() -> Json<Response<Stats>> {
    let mut conn = establish_connection();

    let scans = match scan_info::table
        .order(scan_info::id)
        .load::<ScanInfo>(&mut conn)
    {
        Ok(v) => v,
        Err(e) => {
            return Json(Response::error(ResError {
                msg: e.to_string(),
                detail: String::from("Failed to get scans info!"),
            }))
        }
    };

    let artists = artists::table
        .count()
        .get_result::<i64>(&mut conn)
        .unwrap_or(0);

    let albums = albums::table
        .count()
        .get_result::<i64>(&mut conn)
        .unwrap_or(0);

    let tracks = tracks::table
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
        stats.last_scan = Some(last_scan.clone());
    }

    Json(Response::data(stats))
}
