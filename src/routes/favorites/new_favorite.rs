use super::{FavData, ResError, Response};
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

#[post("/favorites/<track_id>")]
pub fn new_favorite(auth: Authorization, track_id: String) -> Json<Response<FavData>> {
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
