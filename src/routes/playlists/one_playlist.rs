use crate::models::playlists::{Playlist, PlaylistInRes};
use crate::models::tracks::Track;
use crate::models::tracks::TrackInRes;
use crate::routes::prelude::*;
use diesel::dsl::{exists, select};

#[derive(Deserialize, Serialize)]
pub struct Data {
    playlist: PlaylistInRes,
}

#[get("/playlists/<id>")]
pub fn rt(auth: Authorization, id: String) -> Json<Response<Data>> {
    let mut conn = establish_connection();

    if !select(exists(playlists::table.filter(playlists::id.eq(&id))))
        .get_result::<bool>(&mut conn)
        .unwrap()
    {
        return Json(Response::error(ResError {
            msg: "Playlist not found".into(),
            detail: "No such playlist".into(),
        }));
    }

    let playlist: Playlist = match playlists::table
        .filter(playlists::id.eq(&id))
        .get_result::<Playlist>(&mut conn)
    {
        Ok(r) => r,
        Err(e) => {
            return Json(Response::error(ResError {
                msg: "Failed to fetch playlist".into(),
                detail: e.to_string(),
            }))
        }
    };

    if !playlist.is_public && playlist.user_id != auth.user.id {
        return Json(Response::error(ResError {
            msg: "Permission denied".into(),
            detail: "Playlist is not public".into(),
        }));
    }

    Json(Response::data(Data {
        playlist: PlaylistInRes {
            tracks: playlists_tracks::table
                .filter(playlists_tracks::playlist_id.eq(&playlist.id))
                .inner_join(tracks::table)
                .select(Track::as_select())
                .load::<Track>(&mut conn)
                .unwrap()
                .into_iter()
                .map(|t| t.to_response(&mut conn))
                .collect::<Vec<TrackInRes>>(),
            playlist,
        },
    }))
}
