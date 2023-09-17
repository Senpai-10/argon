use super::{FavData, ResError, Response};
use crate::auth::Authorization;
use crate::db;
use crate::models::albums::Album;
use crate::models::features::Feature;
use crate::models::tracks::TrackInRes;
use crate::models::{artists::Artist, tracks::Track};
use crate::schema;
use diesel::prelude::*;
use rocket::serde::json::Json;

#[delete("/favorites/<track_id>")]
pub fn remove_favorite(auth: Authorization, track_id: String) -> Json<Response<FavData>> {
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
