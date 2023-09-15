use super::{ResError, Response};
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
    playlist: PlaylistInRes,
}

#[get("/playlists/<id>")]
pub fn one_playlist(auth: Authorization, id: String) -> Json<Response<Data>> {
    let mut conn = db::establish_connection();

    let playlist: Playlist = match schema::playlists::table
        .filter(schema::playlists::user_id.eq(&auth.user.id))
        .filter(schema::playlists::id.eq(&id))
        .order(schema::playlists::created_at.desc())
        .get_result::<Playlist>(&mut conn)
    {
        Ok(v) => v,
        Err(e) => {
            return Json(Response::error(ResError {
                msg: "Failed to fetch playlist".into(),
                detail: e.to_string(),
            }))
        }
    };

    Json(Response::data(Data {
        playlist: PlaylistInRes {
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
        },
    }))
}
