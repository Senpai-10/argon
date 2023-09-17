mod all_albums;
mod one_album;

use super::Response;

pub fn routes() -> Vec<rocket::Route> {
    routes![all_albums::all_albums, one_album::one_album]
}
