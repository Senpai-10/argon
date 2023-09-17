mod all_favorites;
mod new_favorite;
mod remove_favorite;

use crate::models::tracks::TrackInRes;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct FavData {
    track: TrackInRes,
}

pub fn routes() -> Vec<rocket::Route> {
    routes![all_favorites::rt, new_favorite::rt, remove_favorite::rt,]
}
