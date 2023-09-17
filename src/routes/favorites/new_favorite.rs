use super::FavData;
use crate::models::albums::Album;
use crate::models::favorites::{Favorite, NewFavorite};
use crate::models::features::Feature;
use crate::models::tracks::TrackInRes;
use crate::models::{artists::Artist, tracks::Track};
use crate::routes::prelude::*;
use nanoid::nanoid;

#[post("/favorites/<track_id>")]
pub fn rt(auth: Authorization, track_id: String) -> Json<Response<FavData>> {
    let mut conn = establish_connection();

    if let Ok(track) = tracks::table
        .filter(tracks::id.eq(&track_id))
        .get_result::<Track>(&mut conn)
    {
        if favorites::table
            .filter(favorites::track_id.eq(&track_id))
            .filter(favorites::user_id.eq(&auth.user.id))
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

        if let Err(e) = diesel::insert_into(favorites::table)
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
        }));
    }

    Json(Response::error(ResError {
        msg: "Track does not exists".into(),
        detail: "Track does not exists".into(),
    }))
}
