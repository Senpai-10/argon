use super::Response;
use crate::db;
use crate::models::albums::AlbumWithTracks;
use crate::models::features::Feature;
use crate::models::tracks::TrackInRes;
use crate::models::{albums::Album, artists::Artist, tracks::Track};
use crate::schema;
use diesel::prelude::*;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AlbumData {
    album: AlbumWithTracks,
}

#[get("/albums/<id>")]
pub fn rt(id: String) -> Json<Response<AlbumData>> {
    let mut conn = db::establish_connection();

    let album = schema::albums::table
        .filter(schema::albums::id.eq(id))
        .get_result::<Album>(&mut conn)
        .unwrap();

    let album_tracks = Track::belonging_to(&album)
        .select(Track::as_select())
        .load(&mut conn)
        .unwrap()
        .into_iter()
        .map(|t| TrackInRes {
            artist: t.artist_id.as_ref().map(|artist_id| {
                schema::artists::table
                    .filter(schema::artists::id.eq(artist_id))
                    .get_result(&mut conn)
                    .unwrap()
            }),
            album: Some(album.clone()),
            features: Feature::belonging_to(&t)
                .inner_join(schema::artists::table)
                .select(Artist::as_select())
                .load(&mut conn)
                .unwrap(),
            track: t,
        })
        .collect::<Vec<TrackInRes>>();

    Json(Response::data(AlbumData {
        album: AlbumWithTracks {
            artist: schema::artists::table
                .filter(schema::artists::id.eq(&album.artist_id))
                .get_result::<Artist>(&mut conn)
                .unwrap(),
            album,
            tracks: album_tracks,
        },
    }))
}
