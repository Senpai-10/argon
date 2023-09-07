use crate::db;
use crate::models::tracks::Track;
use crate::schema;
use diesel::prelude::*;
use id3::frame::PictureType;
use rocket::response::status::NotFound;
use std::path::Path;

#[get("/cover/<id>")]
pub fn cover(id: String) -> Result<Vec<u8>, NotFound<String>> {
    let mut conn = db::establish_connection();

    let track = match schema::tracks::table
        .filter(schema::tracks::id.eq(&id))
        .get_result::<Track>(&mut conn)
    {
        Ok(v) => v,
        Err(_) => return Err(NotFound("Track not found!".to_string())),
    };

    let track_file = Path::new(&track.path);

    if !track_file.exists() {
        return Err(NotFound("Track file not found!".to_string()));
    }

    let tag = id3::Tag::read_from_path(track_file).unwrap();

    for pic in tag.pictures() {
        if pic.picture_type == PictureType::CoverFront {
            return Ok(pic.data.to_vec());
        }
    }

    Err(NotFound("Cover not found!".to_string()))
}
