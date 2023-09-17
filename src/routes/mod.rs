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

pub mod prelude {
    pub use crate::auth::Authorization;
    pub use crate::db::establish_connection;
    pub use crate::routes::{ResError, Response};
    pub use crate::schema::*;
    pub use diesel::prelude::*;
    pub use rocket::serde::json::Json;
    pub use serde::{Deserialize, Serialize};
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
    let mut api_routes = routes![stream::rt, picture::rt, stats::rt, scan::rt];

    api_routes.append(&mut auth::routes());
    api_routes.append(&mut playlists::routes());
    api_routes.append(&mut albums::routes());
    api_routes.append(&mut artists::routes());
    api_routes.append(&mut tracks::routes());
    api_routes.append(&mut favorites::routes());
    api_routes.append(&mut search::routes());

    api_routes
}
