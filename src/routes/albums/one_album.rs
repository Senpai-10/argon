use crate::models::albums::AlbumWithTracks;
use crate::models::tracks::TrackInRes;
use crate::models::{albums::Album, artists::Artist, tracks::Track};
use crate::routes::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct AlbumData {
    album: AlbumWithTracks,
}

#[get("/albums/<id>")]
pub fn rt(id: String) -> Json<Response<AlbumData>> {
    let mut conn = establish_connection();

    let album = albums::table
        .filter(albums::id.eq(id))
        .get_result::<Album>(&mut conn)
        .unwrap();

    let album_tracks = Track::belonging_to(&album)
        .select(Track::as_select())
        .load(&mut conn)
        .unwrap()
        .into_iter()
        .map(|t| t.in_response(&mut conn))
        .collect::<Vec<TrackInRes>>();

    Json(Response::data(AlbumData {
        album: AlbumWithTracks {
            artist: artists::table
                .filter(artists::id.eq(&album.artist_id))
                .get_result::<Artist>(&mut conn)
                .unwrap(),
            album,
            tracks: album_tracks,
        },
    }))
}
