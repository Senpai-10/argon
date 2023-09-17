use super::TrackData;
use crate::models::albums::Album;
use crate::models::features::Feature;
use crate::models::tracks::TrackInRes;
use crate::models::{artists::Artist, tracks::Track};
use crate::routes::prelude::*;
use diesel::dsl::{exists, select};

#[delete("/playlists/<id>/<track_id>")]
pub fn remove_playlist_track(
    auth: Authorization,
    id: String,
    track_id: String,
) -> Json<Response<TrackData>> {
    let mut conn = establish_connection();

    if !select(exists(playlists::table.filter(playlists::id.eq(&id))))
        .get_result::<bool>(&mut conn)
        .unwrap()
    {
        return Json(Response::error(ResError {
            msg: "Failed playlist does not exists".into(),
            detail: "Playlist does not exists".into(),
        }));
    }

    if !select(exists(
        playlists::table.filter(playlists::user_id.eq(&auth.user.id)),
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
        playlists_tracks::table
            .filter(playlists_tracks::playlist_id.eq(&id))
            .filter(playlists_tracks::track_id.eq(&track_id)),
    ))
    .get_result::<bool>(&mut conn)
    .unwrap()
    {
        return Json(Response::error(ResError {
            msg: "Track does not exists in playlist".into(),
            detail: "".into(),
        }));
    }

    match tracks::table
        .filter(tracks::id.eq(&track_id))
        .get_result::<Track>(&mut conn)
    {
        Ok(track) => {
            match diesel::delete(
                playlists_tracks::table
                    .filter(playlists_tracks::playlist_id.eq(&id))
                    .filter(playlists_tracks::track_id.eq(&track_id)),
            )
            .execute(&mut conn)
            {
                Ok(_) => Json(Response::data(TrackData {
                    track: TrackInRes {
                        artist: track.artist_id.as_ref().map(|artist_id| {
                            artists::table
                                .filter(artists::id.eq(artist_id))
                                .get_result::<Artist>(&mut conn)
                                .unwrap()
                        }),
                        features: Feature::belonging_to(&track)
                            .inner_join(artists::table)
                            .select(Artist::as_select())
                            .load(&mut conn)
                            .unwrap(),
                        album: track.album_id.as_ref().map(|album_id| {
                            albums::table
                                .filter(albums::id.eq(album_id))
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
