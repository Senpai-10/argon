use crate::models::tracks::Track;
use crate::routes::prelude::*;
use rocket::response::status::NotFound;
use rocket_seek_stream::SeekStream;
use std::path::Path;

#[get("/stream/<id>")]
pub fn rt<'a>(id: String) -> Result<SeekStream<'a>, NotFound<String>> {
    let mut conn = establish_connection();

    let track: Track = match tracks::table
        .filter(tracks::id.eq(&id))
        .get_result::<Track>(&mut conn)
    {
        Ok(v) => v,
        Err(e) => return Err(NotFound(e.to_string())),
    };

    // Update track global plays
    if let Err(e) = diesel::update(tracks::table)
        .filter(tracks::id.eq(&id))
        .set(tracks::plays.eq(track.plays + 1))
        .execute(&mut conn)
    {
        error!("Failed to update track plays!, {e}")
    }

    let file = Path::new(&track.path);

    if file.exists() {
        return match SeekStream::from_path(file) {
            Ok(s) => Ok(s),
            Err(e) => Err(NotFound(e.to_string())),
        };
    }

    Err(NotFound("Track file does not exists!".into()))
}
