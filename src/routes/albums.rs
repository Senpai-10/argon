use super::Response;
use crate::db;
use crate::models::features::Feature;
use crate::models::{albums::Album, artists::Artist, tracks::Track};
use crate::schema;
use diesel::prelude::*;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct TrackWithFeatures {
    #[serde(flatten)]
    track: Track,
    features: Vec<Artist>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AlbumWithTracks {
    #[serde(flatten)]
    album: Album,
    tracks: Vec<TrackWithFeatures>,
}

#[derive(Deserialize, Serialize)]
pub struct Data {
    albums: Vec<AlbumWithTracks>,
    offset: Option<i64>,
    limit: Option<i64>,
    total: i64,
}

#[derive(Serialize, Deserialize)]
pub struct AlbumData {
    album: AlbumWithTracks,
}

#[get("/albums?<artist>&<offset>&<limit>")]
pub fn albums(
    artist: Option<String>,
    offset: Option<i64>,
    limit: Option<i64>,
) -> Json<Response<Data>> {
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
            album,
            tracks: albums_tracks
                .into_iter()
                .map(|t| TrackWithFeatures {
                    features: Feature::belonging_to(&t)
                        .inner_join(schema::artists::table)
                        .select(Artist::as_select())
                        .load(&mut conn)
                        .unwrap(),
                    track: t,
                })
                .collect::<Vec<TrackWithFeatures>>(),
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

#[get("/albums/<id>")]
pub fn album(id: String) -> Json<Response<AlbumData>> {
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
        .map(|t| TrackWithFeatures {
            features: Feature::belonging_to(&t)
                .inner_join(schema::artists::table)
                .select(Artist::as_select())
                .load(&mut conn)
                .unwrap(),
            track: t,
        })
        .collect::<Vec<TrackWithFeatures>>();

    Json(Response::data(AlbumData {
        album: AlbumWithTracks {
            album,
            tracks: album_tracks,
        },
    }))
}
