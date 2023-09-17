mod all_artists;
mod one_artist;

use super::{ResError, Response};

pub fn routes() -> Vec<rocket::Route> {
    routes![all_artists::all_artists, one_artist::one_artist]
}
