use super::{ResError, Response};
use crate::db;
use crate::models::albums::Album;
use crate::models::artists::ArtistWithTracks;
use crate::models::features::Feature;
use crate::models::tracks::TrackInRes;
use crate::models::{artists::Artist, tracks::Track};
use crate::schema;
use diesel::prelude::*;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ArtistData {
    artist: ArtistWithTracks,
}

#[get("/artists/<id>")]
pub fn one_artist(id: String) -> Json<Response<ArtistData>> {
    let mut conn = db::establish_connection();

    let artist: Artist = match schema::artists::table
        .filter(schema::artists::id.eq(&id))
        .select(Artist::as_select())
        .get_result(&mut conn)
    {
        Ok(v) => v,
        Err(e) => {
            return Json(Response::error(ResError {
                msg: e.to_string(),
                detail: format!("Artist '{id}' does not exists!"),
            }))
        }
    };

    let tracks: Vec<TrackInRes> = match Track::belonging_to(&artist)
        .left_join(schema::albums::table)
        .load::<(Track, Option<Album>)>(&mut conn)
    {
        Ok(v) => v
            .into_iter()
            .map(|(t, album)| TrackInRes {
                artist: Some(artist.clone()),
                album,
                features: Feature::belonging_to(&t)
                    .inner_join(schema::artists::table)
                    .select(Artist::as_select())
                    .load(&mut conn)
                    .unwrap(),
                track: t,
            })
            .collect::<Vec<TrackInRes>>(),
        Err(e) => {
            return Json(Response::error(ResError {
                msg: e.to_string(),
                detail: format!("Failed to get tracks for artist '{id}'!"),
            }))
        }
    };

    Json(Response::data(ArtistData {
        artist: ArtistWithTracks { artist, tracks },
    }))
}
