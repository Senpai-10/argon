use crate::models::playlists::{Playlist, PlaylistInRes};
use crate::models::tracks::Track;
use crate::models::tracks::TrackInRes;
use crate::routes::prelude::*;
use rocket::http::Status;
use rocket::request::Outcome;

#[derive(Deserialize, Serialize)]
pub struct Data {
    playlists: Vec<PlaylistInRes>,
    offset: Option<i64>,
    limit: Option<i64>,
    total: i64,
}

#[get("/playlists?<offset>&<limit>")]
pub fn rt(
    auth: Result<Authorization, AuthorizationError>,
    offset: Option<i64>,
    limit: Option<i64>,
) -> Outcome<Data, ResError> {
    if let Err(e) = auth {
        return Outcome::Failure((Status::NonAuthoritativeInformation, e.res_error()));
    }

    let user = auth.unwrap().user;

    let mut conn = establish_connection();
    let mut query = playlists::table.into_boxed();

    if let Some(offset) = offset {
        query = query.offset(offset);
    }

    if let Some(limit) = limit {
        query = query.limit(limit);
    }

    let playlists: Vec<PlaylistInRes> = query
        .filter(playlists::user_id.eq(&user.id))
        .order(playlists::created_at.desc())
        .load::<Playlist>(&mut conn)
        .unwrap()
        .into_iter()
        .map(|playlist| PlaylistInRes {
            tracks: playlists_tracks::table
                .filter(playlists_tracks::playlist_id.eq(&playlist.id))
                .inner_join(tracks::table)
                .select(Track::as_select())
                .load::<Track>(&mut conn)
                .unwrap()
                .into_iter()
                .map(|t| t.in_response(&mut conn))
                .collect::<Vec<TrackInRes>>(),
            playlist,
        })
        .collect::<Vec<PlaylistInRes>>();

    let total = favorites::table
        .count()
        .get_result::<i64>(&mut conn)
        .unwrap();

    Outcome::Success(Data {
        playlists,
        offset,
        limit,
        total,
    })
}
