use super::{get_artists, ArtistsSearchData, Response};
use crate::db;
use rocket::serde::json::Json;

#[get("/search/artists?<q>&<offset>&<limit>")]
pub async fn search_artists(
    q: String,
    offset: Option<i64>,
    limit: Option<i64>,
) -> Json<Response<ArtistsSearchData>> {
    let mut conn = db::establish_connection();

    let search_query = format!("%{q}%");

    Json(Response::data(ArtistsSearchData {
        artists: get_artists(&mut conn, &search_query, offset, limit),
    }))
}
