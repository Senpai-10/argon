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

#[derive(Deserialize, Serialize)]
pub struct Data {
    albums: Vec<AlbumWithTracks>,
    offset: Option<i64>,
    limit: Option<i64>,
    total: i64,
}

#[get("/albums?<artist>&<offset>&<limit>")]
pub fn rt(artist: Option<String>, offset: Option<i64>, limit: Option<i64>) -> Json<Response<Data>> {
    let mut conn = db::establish_connection();
    let mut query = schema::albums::table.into_boxed();

    if let Some(artist) = artist {
        query = query.filter(schema::albums::artist_id.eq(artist));
    }

    if let Some(offset) = offset {
        query = query.offset(offset);
    }

    if let Some(limit) = limit {
        query = query.limit(limit);
    }

    let all_albums = query.select(Album::as_select()).load(&mut conn).unwrap();

    let albums_tracks = Track::belonging_to(&all_albums)
        .select(Track::as_select())
        .load(&mut conn)
        .unwrap();

    let albums_with_tracks = albums_tracks
        .grouped_by(&all_albums)
        .into_iter()
        .zip(all_albums)
        .map(|(albums_tracks, album)| AlbumWithTracks {
            artist: schema::artists::table
                .filter(schema::artists::id.eq(&album.artist_id))
                .get_result::<Artist>(&mut conn)
                .unwrap(),
            tracks: albums_tracks
                .into_iter()
                .map(|t| TrackInRes {
                    artist: Some(
                        schema::artists::table
                            .filter(schema::artists::id.eq(&album.artist_id))
                            .get_result::<Artist>(&mut conn)
                            .unwrap(),
                    ),
                    features: Feature::belonging_to(&t)
                        .inner_join(schema::artists::table)
                        .select(Artist::as_select())
                        .load(&mut conn)
                        .unwrap(),
                    album: Some(album.clone()),
                    track: t,
                })
                .collect::<Vec<TrackInRes>>(),
            album,
        })
        .collect::<Vec<AlbumWithTracks>>();

    let total = schema::albums::table
        .count()
        .get_result::<i64>(&mut conn)
        .unwrap();

    Json(Response::data(Data {
        albums: albums_with_tracks,
        offset,
        limit,
        total,
    }))
}
