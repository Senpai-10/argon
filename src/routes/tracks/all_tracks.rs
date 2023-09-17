use crate::models::albums::Album;
use crate::models::artists::Artist;
use crate::models::features::Feature;
use crate::models::tracks::Track;
use crate::models::tracks::TrackInRes;
use crate::routes::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct TracksData {
    pub tracks: Vec<TrackInRes>,
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub total: i64,
}

#[get("/tracks?<offset>&<limit>")]
pub fn rt(offset: Option<i64>, limit: Option<i64>) -> Json<Response<TracksData>> {
    let mut conn = establish_connection();
    let mut query = tracks::table.into_boxed();

    if let Some(offset) = offset {
        query = query.offset(offset);
    }

    if let Some(limit) = limit {
        query = query.limit(limit);
    }

    let tracks: Vec<TrackInRes> = match query.load::<Track>(&mut conn) {
        Ok(v) => v
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
            .collect::<Vec<TrackInRes>>(),
        Err(e) => {
            return Json(Response::error(ResError {
                msg: e.to_string(),
                detail: String::from("Failed to get tracks"),
            }))
        }
    };

    let total_tracks = tracks::table.count().get_result::<i64>(&mut conn).unwrap();

    Json(Response::data(TracksData {
        tracks,
        offset,
        limit,
        total: total_tracks,
    }))
}
