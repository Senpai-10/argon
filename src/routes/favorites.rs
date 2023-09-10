use super::{ResError, Response};
use crate::auth::Authorization;
use crate::db;
use crate::models::albums::Album;
use crate::models::favorites::{Favorite, NewFavorite};
use crate::models::features::Feature;
use crate::models::tracks::TrackInRes;
use crate::models::{artists::Artist, tracks::Track};
use crate::schema;
use diesel::prelude::*;
use nanoid::nanoid;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Data {
    tracks: Vec<TrackInRes>,
    offset: Option<i64>,
    limit: Option<i64>,
    total: i64,
}

#[derive(Deserialize, Serialize)]
pub struct FavData {
    track: TrackInRes,
}

#[get("/favorites?<offset>&<limit>")]
pub fn favorites(
    auth: Authorization,
    offset: Option<i64>,
    limit: Option<i64>,
) -> Json<Response<Data>> {
    let mut conn = db::establish_connection();
    let mut query = schema::favorites::table.into_boxed();

    if let Some(offset) = offset {
        query = query.offset(offset);
    }

    if let Some(limit) = limit {
        query = query.limit(limit);
    }

    let tracks: Vec<TrackInRes> = query
        .filter(schema::favorites::user_id.eq(&auth.user.id))
        .order(schema::favorites::created_at.desc())
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
        .collect::<Vec<TrackInRes>>();

    let total = schema::favorites::table
        .count()
        .get_result::<i64>(&mut conn)
        .unwrap();

    Json(Response::data(Data {
        tracks,
        offset,
        limit,
        total,
    }))
}

#[post("/favorites/<track_id>")]
pub fn favorite_add(auth: Authorization, track_id: String) -> Json<Response<FavData>> {
    let mut conn = db::establish_connection();

    if let Ok(track) = schema::tracks::table
        .filter(schema::tracks::id.eq(&track_id))
        .get_result::<Track>(&mut conn)
    {
        if schema::favorites::table
            .filter(schema::favorites::track_id.eq(&track_id))
            .filter(schema::favorites::user_id.eq(&auth.user.id))
            .get_result::<Favorite>(&mut conn)
            .is_ok()
        {
            return Json(Response::error(ResError {
                msg: "Already in favorites".into(),
                detail: "Track is already in favorites".into(),
            }));
        }

        let new_favorite = NewFavorite {
            id: nanoid!(),
            user_id: auth.user.id,
            track_id,
        };

        if let Err(e) = diesel::insert_into(schema::favorites::table)
            .values(new_favorite)
            .execute(&mut conn)
        {
            return Json(Response::error(ResError {
                msg: e.to_string(),
                detail: "Failed to add track to favorites".into(),
            }));
        }

        return Json(Response::data(FavData {
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
        }));
    }

    Json(Response::error(ResError {
        msg: "Track does not exists".into(),
        detail: "Track does not exists".into(),
    }))
}

#[delete("/favorites/<track_id>")]
pub fn favorite_remove(auth: Authorization, track_id: String) -> Json<Response<FavData>> {
    let mut conn = db::establish_connection();

    match diesel::delete(
        schema::favorites::table
            .filter(schema::favorites::track_id.eq(&track_id))
            .filter(schema::favorites::user_id.eq(&auth.user.id)),
    )
    .execute(&mut conn)
    {
        Ok(_) => {
            let track = schema::tracks::table
                .filter(schema::tracks::id.eq(&track_id))
                .get_result::<Track>(&mut conn)
                .unwrap();

            Json(Response::data(FavData {
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
            detail: "Track does not exists in favorites".into(),
        })),
    }
}
