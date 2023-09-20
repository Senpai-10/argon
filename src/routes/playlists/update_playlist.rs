use super::PlaylistData;
use crate::models::playlists::Playlist;
use crate::routes::prelude::*;
use diesel::dsl::{exists, select};
use diesel::AsChangeset;

#[derive(Deserialize, Serialize, FromForm, AsChangeset)]
#[diesel(table_name = playlists)]
pub struct UpdatePlaylistBody {
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_public: Option<bool>,
}

#[patch("/playlists/<id>", data = "<body>")]
pub fn rt(
    auth: Authorization,
    id: String,
    body: Json<UpdatePlaylistBody>,
) -> Json<Response<PlaylistData>> {
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
            detail: "You are not allowd to update this playlist".into(),
        }));
    }

    match diesel::update(playlists::table.filter(playlists::id.eq(&id)))
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
