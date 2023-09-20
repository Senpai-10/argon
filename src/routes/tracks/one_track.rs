use crate::models::tracks::Track;
use crate::models::tracks::TrackInRes;
use crate::routes::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct TrackData {
    pub track: TrackInRes,
}

#[get("/tracks/<id>")]
pub fn rt(id: String) -> Json<Response<TrackData>> {
    let mut conn = establish_connection();

    let track = match tracks::table
        .filter(tracks::id.eq(&id))
        .get_result::<Track>(&mut conn)
    {
        Ok(t) => t.in_response(&mut conn),
        Err(e) => {
            return Json(Response::error(ResError {
                msg: e.to_string(),
                detail: format!("Track '{id}' does not exists!"),
            }))
        }
    };

    Json(Response::data(TrackData { track }))
}
