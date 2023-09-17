use crate::models::albums::Album;
use crate::models::features::Feature;
use crate::models::tracks::TrackInRes;
use crate::models::{artists::Artist, tracks::Track};
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
        .map(|t| TrackInRes {
            artist: t.artist_id.as_ref().map(|artist_id| {
                artists::table
                    .filter(artists::id.eq(artist_id))
                    .get_result::<Artist>(&mut conn)
                    .unwrap()
            }),
            features: Feature::belonging_to(&t)
                .inner_join(artists::table)
                .select(Artist::as_select())
                .load(&mut conn)
                .unwrap(),
            album: t.album_id.as_ref().map(|album_id| {
                albums::table
                    .filter(albums::id.eq(album_id))
                    .get_result::<Album>(&mut conn)
                    .unwrap()
            }),
            track: t,
        })
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
