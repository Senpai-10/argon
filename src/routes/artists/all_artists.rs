use super::Response;
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
pub struct Data {
    artists: Vec<ArtistWithTracks>,
    offset: Option<i64>,
    limit: Option<i64>,
    total: i64,
}

#[get("/artists?<offset>&<limit>")]
pub fn rt(offset: Option<i64>, limit: Option<i64>) -> Json<Response<Data>> {
    let mut conn = db::establish_connection();
    let mut query = schema::artists::table.into_boxed();

    if let Some(offset) = offset {
        query = query.offset(offset);
    }

    if let Some(limit) = limit {
        query = query.limit(limit);
    }

    let all_artists = query.select(Artist::as_select()).load(&mut conn).unwrap();

    let tracks: Vec<(Track, Option<Album>)> = Track::belonging_to(&all_artists)
        .left_join(schema::albums::table)
        .load::<(Track, Option<Album>)>(&mut conn)
        .unwrap();

    let artist_with_tracks = tracks
        .grouped_by(&all_artists)
        .into_iter()
        .zip(all_artists)
        .map(|(tracks, artist)| ArtistWithTracks {
            artist: artist.clone(),
            tracks: tracks
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
        })
        .collect::<Vec<ArtistWithTracks>>();

    let total = schema::artists::table
        .count()
        .get_result::<i64>(&mut conn)
        .unwrap();

    Json(Response::data(Data {
        artists: artist_with_tracks,
        offset,
        limit,
        total,
    }))
}
