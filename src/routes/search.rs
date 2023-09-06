use super::Response;
use crate::db;
use crate::models::albums::AlbumWithTracks;
use crate::models::artists::{Artist, ArtistWithTracks};
use crate::models::features::Feature;
use crate::models::tracks::TrackInRes;
use crate::models::{albums::Album, tracks::Track};
use crate::schema;
use diesel::prelude::*;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct SearchAllData {
    artists: Vec<ArtistWithTracks>,
    tracks: Vec<TrackInRes>,
    albums: Vec<AlbumWithTracks>,
}

#[derive(Deserialize, Serialize)]
pub struct TracksSearchData {
    tracks: Vec<TrackInRes>,
}

#[derive(Deserialize, Serialize)]
pub struct ArtistsSearchData {
    artists: Vec<ArtistWithTracks>,
}

#[derive(Deserialize, Serialize)]
pub struct AlbumsSearchData {
    albums: Vec<AlbumWithTracks>,
}

#[get("/search?<q>")]
pub async fn search(q: String) -> Json<Response<SearchAllData>> {
    let mut conn = db::establish_connection();

    let search_query = format!("%{q}%");

    Json(Response::data(SearchAllData {
        artists: get_artists(&mut conn, &search_query),
        tracks: get_tracks(&mut conn, &search_query),
        albums: get_albums(&mut conn, &search_query),
    }))
}

#[get("/search/artists?<q>")]
pub async fn search_artists(q: String) -> Json<Response<ArtistsSearchData>> {
    let mut conn = db::establish_connection();

    let search_query = format!("%{q}%");

    Json(Response::data(ArtistsSearchData {
        artists: get_artists(&mut conn, &search_query),
    }))
}

#[get("/search/tracks?<q>")]
pub async fn search_tracks(q: String) -> Json<Response<TracksSearchData>> {
    let mut conn = db::establish_connection();

    let search_query = format!("%{q}%");

    Json(Response::data(TracksSearchData {
        tracks: get_tracks(&mut conn, &search_query),
    }))
}

#[get("/search/albums?<q>")]
pub async fn search_albums(q: String) -> Json<Response<AlbumsSearchData>> {
    let mut conn = db::establish_connection();

    let search_query = format!("%{q}%");

    Json(Response::data(AlbumsSearchData {
        albums: get_albums(&mut conn, &search_query),
    }))
}

fn get_artists(conn: &mut PgConnection, search_query: &str) -> Vec<ArtistWithTracks> {
    let all_artists = schema::artists::table
        .filter(schema::artists::name.ilike(&search_query))
        .select(Artist::as_select())
        .load(conn)
        .unwrap();

    let tracks: Vec<(Track, Option<Album>)> = Track::belonging_to(&all_artists)
        .left_join(schema::albums::table)
        .load::<(Track, Option<Album>)>(conn)
        .unwrap();

    tracks
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
                        .load(conn)
                        .unwrap(),
                    track: t,
                })
                .collect::<Vec<TrackInRes>>(),
        })
        .collect::<Vec<ArtistWithTracks>>()
}

fn get_tracks(conn: &mut PgConnection, search_query: &str) -> Vec<TrackInRes> {
    schema::tracks::table
        .filter(schema::tracks::title.ilike(&search_query))
        .left_join(schema::artists::table)
        .left_join(schema::albums::table)
        .load::<(Track, Option<Artist>, Option<Album>)>(conn)
        .unwrap()
        .into_iter()
        .map(
            |(track, artist, album): (Track, Option<Artist>, Option<Album>)| TrackInRes {
                artist,
                album,
                features: Feature::belonging_to(&track)
                    .inner_join(schema::artists::table)
                    .select(Artist::as_select())
                    .load(conn)
                    .unwrap(),
                track,
            },
        )
        .collect::<Vec<TrackInRes>>()
}

fn get_albums(conn: &mut PgConnection, search_query: &str) -> Vec<AlbumWithTracks> {
    let all_albums = schema::albums::table
        .filter(schema::albums::title.ilike(&search_query))
        .select(Album::as_select())
        .load(conn)
        .unwrap();

    let albums_tracks = Track::belonging_to(&all_albums)
        .select(Track::as_select())
        .load(conn)
        .unwrap();

    albums_tracks
        .grouped_by(&all_albums)
        .into_iter()
        .zip(all_albums)
        .map(|(albums_tracks, album)| AlbumWithTracks {
            artist: schema::artists::table
                .filter(schema::artists::id.eq(&album.artist_id))
                .get_result::<Artist>(conn)
                .unwrap(),
            album: album.clone(),
            tracks: albums_tracks
                .into_iter()
                .map(|t| TrackInRes {
                    artist: Some(
                        schema::artists::table
                            .filter(schema::artists::id.eq(&album.artist_id))
                            .get_result::<Artist>(conn)
                            .unwrap(),
                    ),
                    album: Some(album.clone()),
                    features: Feature::belonging_to(&t)
                        .inner_join(schema::artists::table)
                        .select(Artist::as_select())
                        .load(conn)
                        .unwrap(),
                    track: t,
                })
                .collect::<Vec<TrackInRes>>(),
        })
        .collect::<Vec<AlbumWithTracks>>()
}
