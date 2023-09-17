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
        stream::stream,
        picture::picture,
        stats::stats,
        scan::scan_route,
    ];

    api_routes.append(&mut auth::routes());
    api_routes.append(&mut playlists::routes());
    api_routes.append(&mut albums::routes());
    api_routes.append(&mut artists::routes());
    api_routes.append(&mut tracks::routes());
    api_routes.append(&mut favorites::routes());
    api_routes.append(&mut search::routes());

    api_routes
}
