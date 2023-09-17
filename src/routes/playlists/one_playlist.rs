use crate::models::albums::Album;
use crate::models::features::Feature;
use crate::models::playlists::{Playlist, PlaylistInRes};
use crate::models::tracks::TrackInRes;
use crate::models::{artists::Artist, tracks::Track};
use crate::routes::prelude::*;

#[derive(Deserialize, Serialize)]
pub struct Data {
    playlist: PlaylistInRes,
}

#[get("/playlists/<id>")]
pub fn one_playlist(auth: Authorization, id: String) -> Json<Response<Data>> {
    let mut conn = establish_connection();

    let playlist: Playlist = match playlists::table
        .filter(playlists::user_id.eq(&auth.user.id))
        .filter(playlists::id.eq(&id))
        .order(playlists::created_at.desc())
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
        },
    }))
}
