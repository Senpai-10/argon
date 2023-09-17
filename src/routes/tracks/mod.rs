mod all_tracks;
mod one_track;

use super::{ResError, Response};

pub fn routes() -> Vec<rocket::Route> {
    routes![all_tracks::tracks, one_track::one_track]
}
