use super::{ResError, Response};
use crate::auth::Authorization;
use crate::db;
use crate::models::albums::Album;
use crate::models::features::Feature;
use crate::models::playlists::{NewPlaylist, Playlist, PlaylistInRes};
use crate::models::playlists_tracks::NewPlaylistTrack;
use crate::models::tracks::TrackInRes;
use crate::models::{artists::Artist, tracks::Track};
use crate::schema;
use diesel::dsl::{exists, select};
use diesel::prelude::*;
use nanoid::nanoid;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Data {
    playlists: Vec<PlaylistInRes>,
    offset: Option<i64>,
    limit: Option<i64>,
    total: i64,
}

#[derive(Deserialize, Serialize)]
pub struct PlaylistData {
    playlist: Playlist,
}

#[derive(Deserialize, Serialize)]
pub struct TrackData {
    track: TrackInRes,
}

#[derive(Deserialize, Serialize, FromForm)]
pub struct NewPlaylistBody {
    pub name: String,
    pub description: Option<String>,
    pub is_public: bool,
}

#[get("/playlists?<offset>&<limit>")]
pub fn playlists(
    auth: Authorization,
    offset: Option<i64>,
    limit: Option<i64>,
) -> Json<Response<Data>> {
    let mut conn = db::establish_connection();
    let mut query = schema::playlists::table.into_boxed();

    if let Some(offset) = offset {
        query = query.offset(offset);
    }

    if let Some(limit) = limit {
        query = query.limit(limit);
    }

    let playlists: Vec<PlaylistInRes> = query
        .filter(schema::playlists::user_id.eq(&auth.user.id))
        .order(schema::playlists::created_at.desc())
        .load::<Playlist>(&mut conn)
        .unwrap()
        .into_iter()
        .map(|playlist| PlaylistInRes {
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
        })
        .collect::<Vec<PlaylistInRes>>();

    let total = schema::favorites::table
        .count()
        .get_result::<i64>(&mut conn)
        .unwrap();

    Json(Response::data(Data {
        playlists,
        offset,
        limit,
        total,
    }))
}

#[delete("/playlists/<id>")]
pub fn playlists_remove(auth: Authorization, id: String) -> Json<Response<PlaylistData>> {
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

#[post("/playlists", data = "<playlist_form>")]
pub fn playlists_new(
    auth: Authorization,
    playlist_form: Json<NewPlaylistBody>,
) -> Json<Response<PlaylistData>> {
    let mut conn = db::establish_connection();

    let new_playlist = NewPlaylist {
        id: nanoid!(),
        user_id: auth.user.id,
        name: playlist_form.name.clone(),
        description: playlist_form.description.clone(),
        is_public: playlist_form.is_public,
    };

    match diesel::insert_into(schema::playlists::table)
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

#[post("/playlists/<id>/<track_id>")]
pub fn playlists_new_track(
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

    match schema::tracks::table
        .filter(schema::tracks::id.eq(&track_id))
        .get_result::<Track>(&mut conn)
    {
        Ok(track) => {
            if select(exists(
                schema::playlists_tracks::table
                    .filter(schema::playlists_tracks::playlist_id.eq(&id))
                    .filter(schema::playlists_tracks::track_id.eq(&track_id)),
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

            if let Err(e) = diesel::insert_into(schema::playlists_tracks::table)
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
            }))
        }
        Err(e) => Json(Response::error(ResError {
            msg: e.to_string(),
            detail: format!("track('{track_id}') does not exists"),
        })),
    }
}

#[delete("/playlists/<id>/<track_id>")]
pub fn playlists_remove_track(
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
