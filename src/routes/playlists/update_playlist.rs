use super::PlaylistData;
use super::{ResError, Response};
use crate::auth::Authorization;
use crate::db;
use crate::models::playlists::Playlist;
use crate::schema;
use diesel::dsl::{exists, select};
use diesel::prelude::*;
use diesel::AsChangeset;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, FromForm, AsChangeset)]
#[diesel(table_name = schema::playlists)]
pub struct UpdatePlaylistBody {
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_public: Option<bool>,
}

#[patch("/playlists/<id>", data = "<body>")]
pub fn update_playlist(
    auth: Authorization,
    id: String,
    body: Json<UpdatePlaylistBody>,
) -> Json<Response<PlaylistData>> {
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
            detail: "You are not allowd to update this playlist".into(),
        }));
    }

    match diesel::update(schema::playlists::table.filter(schema::playlists::id.eq(&id)))
        .set::<UpdatePlaylistBody>(body.into_inner())
        .get_result::<Playlist>(&mut conn)
    {
        Ok(playlist) => Json(Response::data(PlaylistData { playlist })),
        Err(e) => Json(Response::error(ResError {
            msg: e.to_string(),
            detail: "".into(),
        })),
    }
}
