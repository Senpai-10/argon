use crate::models::tracks::Track;
use crate::routes::prelude::*;
use id3::frame::PictureType;
use rocket::response::status::NotFound;
use std::path::Path;

#[derive(Debug, PartialEq, Eq, FromFormField)]
#[allow(non_camel_case_types)]
pub enum PicType {
    other,
    icon,
    other_icon,
    cover_front,
    cover_back,
    leaflet,
    media,
    lead_artist,
    artist,
    conductor,
    band,
    composer,
    lyricist,
    recording_location,
    during_recording,
    during_performance,
    screen_capture,
    bright_fish,
    illustration,
    band_logo,
    publisher_logo,
}

#[get("/picture/<id>?<pic_type>")]
pub fn rt(id: String, pic_type: Option<PicType>) -> Result<Vec<u8>, NotFound<String>> {
    let mut conn = establish_connection();
    let pic_type = pic_type.unwrap_or(PicType::cover_front);

    let track = match tracks::table
        .filter(tracks::id.eq(&id))
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
            (PictureType::Other, PicType::other)
            | (PictureType::Icon, PicType::icon)
            | (PictureType::OtherIcon, PicType::other_icon)
            | (PictureType::CoverFront, PicType::cover_front)
            | (PictureType::CoverBack, PicType::cover_back)
            | (PictureType::Leaflet, PicType::leaflet)
            | (PictureType::Media, PicType::media)
            | (PictureType::LeadArtist, PicType::lead_artist)
            | (PictureType::Artist, PicType::artist)
            | (PictureType::Conductor, PicType::conductor)
            | (PictureType::Band, PicType::band)
            | (PictureType::Composer, PicType::composer)
            | (PictureType::Lyricist, PicType::lyricist)
            | (PictureType::RecordingLocation, PicType::recording_location)
            | (PictureType::DuringRecording, PicType::during_recording)
            | (PictureType::DuringPerformance, PicType::during_performance)
            | (PictureType::ScreenCapture, PicType::screen_capture)
            | (PictureType::BrightFish, PicType::bright_fish)
            | (PictureType::Illustration, PicType::illustration)
            | (PictureType::BandLogo, PicType::band_logo)
            | (PictureType::PublisherLogo, PicType::publisher_logo) => {
                return Ok(pic.data.to_vec())
            }
            _ => {}
        }
    }

    Err(NotFound("Picture does not exists!".to_string()))
}
