mod search_albums;
mod search_all;
mod search_artists;
mod search_tracks;

use crate::models::albums::AlbumWithTracks;
use crate::models::artists::{Artist, ArtistWithTracks};
use crate::models::tracks::TrackInRes;
use crate::models::{albums::Album, tracks::Track};
use crate::schema;
use diesel::prelude::*;
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

pub fn get_artists(
    conn: &mut PgConnection,
    search_query: &str,
    offset: Option<i64>,
    limit: Option<i64>,
) -> Vec<ArtistWithTracks> {
    let mut query = schema::artists::table.into_boxed();

    if let Some(offset) = offset {
        query = query.offset(offset);
    }

    if let Some(limit) = limit {
        query = query.limit(limit);
    }

    let all_artists = query
        .filter(schema::artists::name.ilike(&search_query))
        .select(Artist::as_select())
        .load(conn)
        .unwrap();

    let tracks: Vec<Track> = Track::belonging_to(&all_artists)
        .load::<Track>(conn)
        .unwrap();

    tracks
        .grouped_by(&all_artists)
        .into_iter()
        .zip(all_artists)
        .map(|(tracks, artist)| ArtistWithTracks {
            artist: artist.clone(),
            tracks: tracks
                .into_iter()
                .map(|t| t.in_response(conn))
                .collect::<Vec<TrackInRes>>(),
            featured_on: schema::features::table
                .filter(schema::features::artist_id.eq(&artist.id))
                .inner_join(schema::tracks::table)
                .select(Track::as_select())
                .load(conn)
                .unwrap()
                .into_iter()
                .map(|track| track.in_response(conn))
                .collect::<Vec<TrackInRes>>(),
        })
        .collect::<Vec<ArtistWithTracks>>()
}

pub fn get_tracks(
    conn: &mut PgConnection,
    search_query: &str,
    offset: Option<i64>,
    limit: Option<i64>,
) -> Vec<TrackInRes> {
    let mut query = schema::tracks::table
        .filter(schema::tracks::title.ilike(&search_query))
        .into_boxed();

    if let Some(offset) = offset {
        query = query.offset(offset);
    }

    if let Some(limit) = limit {
        query = query.limit(limit);
    }

    query
        .load::<Track>(conn)
        .unwrap()
        .into_iter()
        .map(|track: Track| track.in_response(conn))
        .collect::<Vec<TrackInRes>>()
}

pub fn get_albums(
    conn: &mut PgConnection,
    search_query: &str,
    offset: Option<i64>,
    limit: Option<i64>,
) -> Vec<AlbumWithTracks> {
    let mut query = schema::albums::table.into_boxed();

    if let Some(offset) = offset {
        query = query.offset(offset);
    }

    if let Some(limit) = limit {
        query = query.limit(limit);
    }

    let all_albums = query
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
                .map(|t| t.in_response(conn))
                .collect::<Vec<TrackInRes>>(),
        })
        .collect::<Vec<AlbumWithTracks>>()
}

pub fn routes() -> Vec<rocket::Route> {
    routes![
        search_all::rt,
        search_artists::rt,
        search_tracks::rt,
        search_albums::rt,
    ]
}
