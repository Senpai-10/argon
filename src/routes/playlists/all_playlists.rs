use crate::models::albums::Album;
use crate::models::features::Feature;
use crate::models::playlists::{Playlist, PlaylistInRes};
use crate::models::tracks::TrackInRes;
use crate::models::{artists::Artist, tracks::Track};
use crate::routes::prelude::*;

#[derive(Deserialize, Serialize)]
pub struct Data {
    playlists: Vec<PlaylistInRes>,
    offset: Option<i64>,
    limit: Option<i64>,
    total: i64,
}

#[get("/playlists?<offset>&<limit>")]
pub fn all_playlists(
    auth: Authorization,
    offset: Option<i64>,
    limit: Option<i64>,
) -> Json<Response<Data>> {
    let mut conn = establish_connection();
    let mut query = playlists::table.into_boxed();

    if let Some(offset) = offset {
        query = query.offset(offset);
    }

    if let Some(limit) = limit {
        query = query.limit(limit);
    }

    let playlists: Vec<PlaylistInRes> = query
        .filter(playlists::user_id.eq(&auth.user.id))
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
            playlist,
        })
        .collect::<Vec<PlaylistInRes>>();

    let total = favorites::table
        .count()
        .get_result::<i64>(&mut conn)
        .unwrap();

    Json(Response::data(Data {
        playlists,
        offset,
        limit,
        total,
    }))
}
