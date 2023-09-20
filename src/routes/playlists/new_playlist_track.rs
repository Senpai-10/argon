use super::TrackData;
use crate::models::albums::Album;
use crate::models::features::Feature;
use crate::models::playlists_tracks::NewPlaylistTrack;
use crate::models::tracks::TrackInRes;
use crate::models::{artists::Artist, tracks::Track};
use crate::routes::prelude::*;
use diesel::dsl::{exists, select};
use nanoid::nanoid;

#[post("/playlists/<id>/<track_id>")]
pub fn rt(auth: Authorization, id: String, track_id: String) -> Json<Response<TrackData>> {
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

    match tracks::table
        .filter(tracks::id.eq(&track_id))
        .get_result::<Track>(&mut conn)
    {
        Ok(track) => {
            if select(exists(
                playlists_tracks::table
                    .filter(playlists_tracks::playlist_id.eq(&id))
                    .filter(playlists_tracks::track_id.eq(&track_id)),
            ))
            .get_result::<bool>(&mut conn)
            .unwrap()
            {
                return Json(Response::error(ResError {
                    msg: "Failed to add track".into(),
                    detail: "Track already exists in playlist".into(),
                }));
            }

            let new_playlist_track = NewPlaylistTrack {
                id: nanoid!(),
                playlist_id: id.clone(),
                track_id: track_id.clone(),
            };

            if let Err(e) = diesel::insert_into(playlists_tracks::table)
                .values(new_playlist_track)
                .execute(&mut conn)
            {
                return Json(Response::error(ResError {
                    msg: e.to_string(),
                    detail: format!("Failed to add track('{}') to playlist('{}')", track_id, id),
                }));
            };

            Json(Response::data(TrackData {
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
            }))
        }
        Err(e) => Json(Response::error(ResError {
            msg: e.to_string(),
            detail: format!("track('{track_id}') does not exists"),
        })),
    }
}
