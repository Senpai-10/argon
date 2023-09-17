mod all_artists;
mod one_artist;

pub fn routes() -> Vec<rocket::Route> {
    routes![all_artists::rt, one_artist::rt]
}
