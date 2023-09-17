mod all_artists;
mod one_artist;

use super::{ResError, Response};

pub fn routes() -> Vec<rocket::Route> {
    routes![all_artists::rt, one_artist::rt]
}
