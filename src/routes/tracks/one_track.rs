use super::{ResError, Response};
use crate::models::albums::Album;
use crate::models::artists::Artist;
use crate::models::features::Feature;
use crate::models::tracks::Track;
use crate::schema;
use crate::{db, models::tracks::TrackInRes};
use diesel::prelude::*;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TrackData {
    pub track: TrackInRes,
}

#[get("/tracks/<id>")]
pub fn one_track(id: String) -> Json<Response<TrackData>> {
    let mut conn = db::establish_connection();

    let track = match schema::tracks::table
        .filter(schema::tracks::id.eq(&id))
        .get_result::<Track>(&mut conn)
    {
        Ok(t) => TrackInRes {
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
        },
        Err(e) => {
            return Json(Response::error(ResError {
                msg: e.to_string(),
                detail: format!("Track '{id}' does not exists!"),
            }))
        }
    };

    Json(Response::data(TrackData { track }))
}
