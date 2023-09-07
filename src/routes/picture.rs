use crate::db;
use crate::models::tracks::Track;
use crate::schema;
use diesel::prelude::*;
use id3::frame::PictureType;
use rocket::response::status::NotFound;
use std::path::Path;

#[derive(Debug, PartialEq, Eq, FromFormField)]
pub enum PicType {
    Other,
    Icon,
    OtherIcon,
    CoverFront,
    CoverBack,
    Leaflet,
    Media,
    LeadArtist,
    Artist,
    Conductor,
    Band,
    Composer,
    Lyricist,
    RecordingLocation,
    DuringRecording,
    DuringPerformance,
    ScreenCapture,
    BrightFish,
    Illustration,
    BandLogo,
    PublisherLogo,
}

#[get("/picture/<id>?<pic_type>")]
pub fn picture(id: String, pic_type: Option<PicType>) -> Result<Vec<u8>, NotFound<String>> {
    let mut conn = db::establish_connection();
    let pic_type = pic_type.unwrap_or(PicType::CoverFront);

    let track = match schema::tracks::table
        .filter(schema::tracks::id.eq(&id))
        .get_result::<Track>(&mut conn)
    {
        Ok(v) => v,
        Err(_) => return Err(NotFound("Track does not exists!".to_string())),
    };

    let track_file = Path::new(&track.path);

    if !track_file.exists() {
        return Err(NotFound("Track file does not exists!".to_string()));
    }

    let tag = id3::Tag::read_from_path(track_file).unwrap();

    for pic in tag.pictures() {
        match (pic.picture_type, &pic_type) {
            (PictureType::Other, PicType::Other)
            | (PictureType::Icon, PicType::Icon)
            | (PictureType::OtherIcon, PicType::OtherIcon)
            | (PictureType::CoverFront, PicType::CoverFront)
            | (PictureType::CoverBack, PicType::CoverBack)
            | (PictureType::Leaflet, PicType::Leaflet)
            | (PictureType::Media, PicType::Media)
            | (PictureType::LeadArtist, PicType::LeadArtist)
            | (PictureType::Artist, PicType::Artist)
            | (PictureType::Conductor, PicType::Conductor)
            | (PictureType::Band, PicType::Band)
            | (PictureType::Composer, PicType::Composer)
            | (PictureType::Lyricist, PicType::Lyricist)
            | (PictureType::RecordingLocation, PicType::RecordingLocation)
            | (PictureType::DuringRecording, PicType::DuringRecording)
            | (PictureType::DuringPerformance, PicType::DuringPerformance)
            | (PictureType::ScreenCapture, PicType::ScreenCapture)
            | (PictureType::BrightFish, PicType::BrightFish)
            | (PictureType::Illustration, PicType::Illustration)
            | (PictureType::BandLogo, PicType::BandLogo)
            | (PictureType::PublisherLogo, PicType::PublisherLogo) => return Ok(pic.data.to_vec()),
            _ => {}
        }
    }

    Err(NotFound("Picture does not exists!".to_string()))
}
