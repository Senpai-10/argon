use super::Response;
use crate::auth::Authorization;
use crate::db;
use crate::models::albums::Album;
use crate::models::features::Feature;
use crate::models::playlists::{Playlist, PlaylistInRes};
use crate::models::tracks::TrackInRes;
use crate::models::{artists::Artist, tracks::Track};
use crate::schema;
use diesel::prelude::*;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

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
    let mut conn = db::establish_connection();
    let mut query = schema::playlists::table.into_boxed();

    if let Some(offset) = offset {
        query = query.offset(offset);
    }

    if let Some(limit) = limit {
        query = query.limit(limit);
    }

    let playlists: Vec<PlaylistInRes> = query
        .filter(schema::playlists::user_id.eq(&auth.user.id))
        .order(schema::playlists::created_at.desc())
        .load::<Playlist>(&mut conn)
        .unwrap()
        .into_iter()
        .map(|playlist| PlaylistInRes {
            tracks: schema::playlists_tracks::table
                .filter(schema::playlists_tracks::playlist_id.eq(&playlist.id))
                .inner_join(schema::tracks::table)
                .select(Track::as_select())
                .load::<Track>(&mut conn)
                .unwrap()
                .into_iter()
                .map(|t| TrackInRes {
                    artist: t.artist_id.as_ref().map(|artist_id| {
                        schema::artists::table
                            .filter(schema::artists::id.eq(artist_id))
                            .get_result::<Artist>(&mut conn)
                            .unwrap()
                    }),
                    features: Feature::belonging_to(&t)
                        .inner_join(schema::artists::table)
                        .select(Artist::as_select())
                        .load(&mut conn)
                        .unwrap(),
                    album: t.album_id.as_ref().map(|album_id| {
                        schema::albums::table
                            .filter(schema::albums::id.eq(album_id))
                            .get_result::<Album>(&mut conn)
                            .unwrap()
                    }),
                    track: t,
                })
                .collect::<Vec<TrackInRes>>(),
            playlist,
        })
        .collect::<Vec<PlaylistInRes>>();

    let total = schema::favorites::table
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
