use super::{ResError, Response, TrackData};
use crate::auth::Authorization;
use crate::db;
use crate::models::albums::Album;
use crate::models::features::Feature;
use crate::models::tracks::TrackInRes;
use crate::models::{artists::Artist, tracks::Track};
use crate::schema;
use diesel::dsl::{exists, select};
use diesel::prelude::*;
use rocket::serde::json::Json;

#[delete("/playlists/<id>/<track_id>")]
pub fn remove_playlist_track(
    auth: Authorization,
    id: String,
    track_id: String,
) -> Json<Response<TrackData>> {
    let mut conn = db::establish_connection();

    if !select(exists(
        schema::playlists::table.filter(schema::playlists::id.eq(&id)),
    ))
    .get_result::<bool>(&mut conn)
    .unwrap()
    {
        return Json(Response::error(ResError {
            msg: "Failed playlist does not exists".into(),
            detail: "Playlist does not exists".into(),
        }));
    }

    if !select(exists(
        schema::playlists::table.filter(schema::playlists::user_id.eq(&auth.user.id)),
    ))
    .get_result::<bool>(&mut conn)
    .unwrap()
    {
        return Json(Response::error(ResError {
            msg: "Permission denied".into(),
            detail: "You are not allowd to add a track to this playlist".into(),
        }));
    }

    if !select(exists(
        schema::playlists_tracks::table
            .filter(schema::playlists_tracks::playlist_id.eq(&id))
            .filter(schema::playlists_tracks::track_id.eq(&track_id)),
    ))
    .get_result::<bool>(&mut conn)
    .unwrap()
    {
        return Json(Response::error(ResError {
            msg: "Track does not exists in playlist".into(),
            detail: "".into(),
        }));
    }

    match schema::tracks::table
        .filter(schema::tracks::id.eq(&track_id))
        .get_result::<Track>(&mut conn)
    {
        Ok(track) => {
            match diesel::delete(
                schema::playlists_tracks::table
                    .filter(schema::playlists_tracks::playlist_id.eq(&id))
                    .filter(schema::playlists_tracks::track_id.eq(&track_id)),
            )
            .execute(&mut conn)
            {
                Ok(_) => Json(Response::data(TrackData {
                    track: TrackInRes {
                        artist: track.artist_id.as_ref().map(|artist_id| {
                            schema::artists::table
                                .filter(schema::artists::id.eq(artist_id))
                                .get_result::<Artist>(&mut conn)
                                .unwrap()
                        }),
                        features: Feature::belonging_to(&track)
                            .inner_join(schema::artists::table)
                            .select(Artist::as_select())
                            .load(&mut conn)
                            .unwrap(),
                        album: track.album_id.as_ref().map(|album_id| {
                            schema::albums::table
                                .filter(schema::albums::id.eq(album_id))
                                .get_result::<Album>(&mut conn)
                                .unwrap()
                        }),
                        track,
                    },
                })),
                Err(e) => Json(Response::error(ResError {
                    msg: "Failed to remove track from playlist".into(),
                    detail: e.to_string(),
                })),
            }
        }
        Err(e) => Json(Response::error(ResError {
            msg: "Track does not exists".into(),
            detail: e.to_string(),
        })),
    }
}
