use super::PlaylistData;
use crate::models::playlists::{NewPlaylist, Playlist};
use crate::routes::prelude::*;
use nanoid::nanoid;

#[derive(Deserialize, Serialize, FromForm)]
pub struct NewPlaylistBody {
    pub name: String,
    pub description: Option<String>,
    pub is_public: bool,
}

#[post("/playlists", data = "<playlist_form>")]
pub fn new_playlist(
    auth: Authorization,
    playlist_form: Json<NewPlaylistBody>,
) -> Json<Response<PlaylistData>> {
    let mut conn = establish_connection();

    let new_playlist = NewPlaylist {
        id: nanoid!(),
        user_id: auth.user.id,
        name: playlist_form.name.clone(),
        description: playlist_form.description.clone(),
        is_public: playlist_form.is_public,
    };

    match diesel::insert_into(playlists::table)
        .values(new_playlist)
        .get_result::<Playlist>(&mut conn)
    {
        Ok(playlist) => Json(Response::data(PlaylistData { playlist })),
        Err(e) => Json(Response::error(ResError {
            msg: e.to_string(),
            detail: "".into(),
        })),
    }
}
