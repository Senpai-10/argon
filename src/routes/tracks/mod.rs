mod all_tracks;
mod one_track;

use super::{ResError, Response};

pub fn routes() -> Vec<rocket::Route> {
    routes![all_tracks::rt, one_track::rt]
}
