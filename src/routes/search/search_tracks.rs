use super::{get_tracks, TracksSearchData};
use crate::routes::prelude::*;

#[get("/search/tracks?<q>&<offset>&<limit>")]
pub async fn rt(
    q: String,
    offset: Option<i64>,
    limit: Option<i64>,
) -> Json<Response<TracksSearchData>> {
    let mut conn = establish_connection();

    let search_query = format!("%{q}%");

    Json(Response::data(TracksSearchData {
        tracks: get_tracks(&mut conn, &search_query, offset, limit),
    }))
}
