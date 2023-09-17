use super::{get_albums, AlbumsSearchData};
use crate::routes::prelude::*;

#[get("/search/albums?<q>&<offset>&<limit>")]
pub async fn rt(
    q: String,
    offset: Option<i64>,
    limit: Option<i64>,
) -> Json<Response<AlbumsSearchData>> {
    let mut conn = establish_connection();

    let search_query = format!("%{q}%");

    Json(Response::data(AlbumsSearchData {
        albums: get_albums(&mut conn, &search_query, offset, limit),
    }))
}
