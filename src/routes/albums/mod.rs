mod all_albums;
mod one_album;

pub fn routes() -> Vec<rocket::Route> {
    routes![all_albums::rt, one_album::rt]
}
