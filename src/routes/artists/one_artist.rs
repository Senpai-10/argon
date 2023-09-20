use crate::models::albums::Album;
use crate::models::artists::ArtistWithTracks;
use crate::models::features::Feature;
use crate::models::tracks::TrackInRes;
use crate::models::{artists::Artist, tracks::Track};
use crate::routes::prelude::*;

#[derive(Deserialize, Serialize)]
pub struct ArtistData {
    artist: ArtistWithTracks,
}

#[get("/artists/<id>")]
pub fn rt(id: String) -> Json<Response<ArtistData>> {
    let mut conn = establish_connection();

    let artist: Artist = match artists::table
        .filter(artists::id.eq(&id))
        .select(Artist::as_select())
        .get_result(&mut conn)
    {
        Ok(v) => v,
        Err(e) => {
            return Json(Response::error(ResError {
                msg: e.to_string(),
                detail: format!("Artist '{id}' does not exists!"),
            }))
        }
    };

    let tracks: Vec<TrackInRes> = match Track::belonging_to(&artist)
        .left_join(albums::table)
        .load::<(Track, Option<Album>)>(&mut conn)
    {
        Ok(v) => v
            .into_iter()
            .map(|(t, album)| TrackInRes {
                artist: Some(artist.clone()),
                album,
                features: Feature::belonging_to(&t)
                    .inner_join(artists::table)
                    .select(Artist::as_select())
                    .load(&mut conn)
                    .unwrap(),
                track: t,
            })
            .collect::<Vec<TrackInRes>>(),
        Err(e) => {
            return Json(Response::error(ResError {
                msg: e.to_string(),
                detail: format!("Failed to get tracks for artist '{id}'!"),
            }))
        }
    };

    Json(Response::data(ArtistData {
        artist: ArtistWithTracks {
            tracks,
            featured_on: features::table
                .filter(features::artist_id.eq(&artist.id))
                .inner_join(tracks::table)
                .select(Track::as_select())
                .load(&mut conn)
                .unwrap()
                .into_iter()
                .map(|track| TrackInRes {
                    artist: Some(artist.clone()),
                    album: track.album_id.as_ref().map(|album_id| {
                        albums::table
                            .filter(albums::id.eq(album_id))
                            .get_result::<Album>(&mut conn)
                            .unwrap()
                    }),
                    features: Feature::belonging_to(&track)
                        .inner_join(artists::table)
                        .select(Artist::as_select())
                        .load(&mut conn)
                        .unwrap(),
                    track,
                })
                .collect::<Vec<TrackInRes>>(),
            artist,
        },
    }))
}
