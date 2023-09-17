use super::{get_albums, get_artists, get_tracks, Response, SearchAllData};
use crate::db;
use rocket::serde::json::Json;

#[get("/search?<q>&<offset>&<limit>")]
pub async fn search_all(
    q: String,
    offset: Option<i64>,
    limit: Option<i64>,
) -> Json<Response<SearchAllData>> {
    let mut conn = db::establish_connection();

    let search_query = format!("%{q}%");

    Json(Response::data(SearchAllData {
        artists: get_artists(&mut conn, &search_query, offset, limit),
        tracks: get_tracks(&mut conn, &search_query, offset, limit),
        albums: get_albums(&mut conn, &search_query, offset, limit),
    }))
}
