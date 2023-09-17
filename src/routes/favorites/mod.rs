mod all_favorites;
mod new_favorite;
mod remove_favorite;

use super::{ResError, Response};
use crate::models::tracks::TrackInRes;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct FavData {
    track: TrackInRes,
}

pub fn routes() -> Vec<rocket::Route> {
    routes![
        all_favorites::all_favorites,
        new_favorite::new_favorite,
        remove_favorite::remove_favorite,
    ]
}
