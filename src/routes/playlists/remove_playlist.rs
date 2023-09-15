use super::{PlaylistData, ResError, Response};
use crate::auth::Authorization;
use crate::db;
use crate::models::playlists::Playlist;
use crate::schema;
use diesel::dsl::{exists, select};
use diesel::prelude::*;
use rocket::serde::json::Json;

#[delete("/playlists/<id>")]
pub fn remove_playlist(auth: Authorization, id: String) -> Json<Response<PlaylistData>> {
    let mut conn = db::establish_connection();

    if !select(exists(
        schema::playlists::table.filter(schema::playlists::user_id.eq(&auth.user.id)),
    ))
    .get_result::<bool>(&mut conn)
    .unwrap()
    {
        return Json(Response::error(ResError {
            msg: "Permission denied".into(),
            detail: "You are not allowd to remove this playlist".into(),
        }));
    }

    let delete_statment: Result<Playlist, diesel::result::Error> =
        diesel::delete(schema::playlists::table.filter(schema::playlists::id.eq(&id)))
            .get_result::<Playlist>(&mut conn);

    match delete_statment {
        Ok(playlist) => Json(Response::data(PlaylistData { playlist })),
        Err(e) => Json(Response::error(ResError {
            msg: "Failed to delete playlist".into(),
            detail: e.to_string(),
        })),
    }
}
