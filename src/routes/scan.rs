use crate::models::scan_info::ScanInfo;
use crate::routes::prelude::*;

use crate::scan::Scanner;

#[derive(Deserialize, Serialize)]
pub struct Data {
    scan_info: Option<ScanInfo>,
}

#[get("/scan?<clean>")]
pub async fn rt(clean: Option<bool>) -> Json<Response<Data>> {
    let mut conn = establish_connection();

    if let Some(true) = clean {
        match diesel::delete(artists::table).execute(&mut conn) {
            Ok(v) => info!("Removed {v} artist!"),
            Err(e) => error!("Failed to clear artists table! {e}"),
        };

        match diesel::delete(albums::table).execute(&mut conn) {
            Ok(v) => info!("Removed {v} album!"),
            Err(e) => error!("Failed to clear albums table! {e}"),
        };

        match diesel::delete(features::table).execute(&mut conn) {
            Ok(v) => info!("Removed {v} feature!"),
            Err(e) => error!("Failed to clear features table! {e}"),
        }

        match diesel::delete(tracks::table).execute(&mut conn) {
            Ok(v) => info!("Removed {v} track!"),
            Err(e) => error!("Failed to clear tracks table! {e}"),
        };
    }

    let mut scanner = Scanner::new(conn);

    let mut data = Data { scan_info: None };

    match scanner.start() {
        Ok(v) => data.scan_info = v,
        Err(e) => {
            return Json(Response::error(ResError {
                msg: e.to_string(),
                detail: e.to_string(),
            }))
        }
    }

    Json(Response::data(data))
}
