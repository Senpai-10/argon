use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ResError {
    pub msg: String,
    pub detail: String,
}

#[allow(non_camel_case_types)]
#[derive(Deserialize, Serialize)]
pub enum Response<T> {
    data(T),
    error(ResError),
}

mod albums;
mod artists;
mod auth;
mod favorites;
mod picture;
mod playlists;
mod scan;
mod search;
mod stats;
mod stream;
mod tracks;

pub fn routes() -> Vec<rocket::Route> {
    let mut api_routes = routes![
        tracks::tracks,
        tracks::track,
        stream::stream,
        picture::picture,
        artists::artists,
        artists::artist,
        albums::albums,
        albums::album,
        stats::stats,
        scan::scan_route,
        search::search,
        search::search_artists,
        search::search_tracks,
        search::search_albums,
        favorites::favorites,
        favorites::favorite_add,
        favorites::favorite_remove,
        auth::signup,
        auth::login,
        auth::logout,
    ];

    api_routes.append(&mut playlists::routes());

    api_routes
}
