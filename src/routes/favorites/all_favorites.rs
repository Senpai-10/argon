use crate::models::tracks::Track;
use crate::models::tracks::TrackInRes;
use crate::routes::prelude::*;

#[derive(Deserialize, Serialize)]
pub struct Data {
    tracks: Vec<TrackInRes>,
    offset: Option<i64>,
    limit: Option<i64>,
    total: i64,
}

#[get("/favorites?<offset>&<limit>")]
pub fn rt(auth: Authorization, offset: Option<i64>, limit: Option<i64>) -> Json<Response<Data>> {
    let mut conn = establish_connection();
    let mut query = favorites::table.into_boxed();

    if let Some(offset) = offset {
        query = query.offset(offset);
    }

    if let Some(limit) = limit {
        query = query.limit(limit);
    }

    let tracks: Vec<TrackInRes> = query
        .filter(favorites::user_id.eq(&auth.user.id))
        .order(favorites::created_at.desc())
        .inner_join(tracks::table)
        .select(Track::as_select())
        .load::<Track>(&mut conn)
        .unwrap()
        .into_iter()
        .map(|t| t.in_response(&mut conn))
        .collect::<Vec<TrackInRes>>();

    let total = favorites::table
        .count()
        .get_result::<i64>(&mut conn)
        .unwrap();

    Json(Response::data(Data {
        tracks,
        offset,
        limit,
        total,
    }))
}
