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

pub mod albums;
pub mod artists;
pub mod auth;
pub mod favorites;
pub mod picture;
pub mod playlists;
pub mod scan;
pub mod search;
pub mod stats;
pub mod stream;
pub mod tracks;
