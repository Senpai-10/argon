use super::{get_artists, ArtistsSearchData};
use crate::routes::prelude::*;

#[get("/search/artists?<q>&<offset>&<limit>")]
pub async fn rt(
    q: String,
    offset: Option<i64>,
    limit: Option<i64>,
) -> Json<Response<ArtistsSearchData>> {
    let mut conn = establish_connection();

    let search_query = format!("%{q}%");

    Json(Response::data(ArtistsSearchData {
        artists: get_artists(&mut conn, &search_query, offset, limit),
    }))
}
