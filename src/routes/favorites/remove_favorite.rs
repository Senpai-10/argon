use super::FavData;
use crate::models::albums::Album;
use crate::models::features::Feature;
use crate::models::tracks::TrackInRes;
use crate::models::{artists::Artist, tracks::Track};
use crate::routes::prelude::*;

#[delete("/favorites/<track_id>")]
pub fn rt(auth: Authorization, track_id: String) -> Json<Response<FavData>> {
    let mut conn = establish_connection();

    match diesel::delete(
        favorites::table
            .filter(favorites::track_id.eq(&track_id))
            .filter(favorites::user_id.eq(&auth.user.id)),
    )
    .execute(&mut conn)
    {
        Ok(_) => {
            let track = tracks::table
                .filter(tracks::id.eq(&track_id))
                .get_result::<Track>(&mut conn)
                .unwrap();

            Json(Response::data(FavData {
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
            }))
        }
        Err(e) => Json(Response::error(ResError {
            msg: e.to_string(),
            detail: "Track does not exists in favorites".into(),
        })),
    }
}
