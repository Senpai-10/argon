use super::{get_albums, AlbumsSearchData, Response};
use crate::db;
use rocket::serde::json::Json;

#[get("/search/albums?<q>&<offset>&<limit>")]
pub async fn rt(
    q: String,
    offset: Option<i64>,
    limit: Option<i64>,
) -> Json<Response<AlbumsSearchData>> {
    let mut conn = db::establish_connection();

    let search_query = format!("%{q}%");

    Json(Response::data(AlbumsSearchData {
        albums: get_albums(&mut conn, &search_query, offset, limit),
    }))
}
