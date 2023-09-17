use crate::models::scan_info::ScanInfo;
use crate::routes::prelude::*;

#[derive(Deserialize, Serialize)]
pub struct Data {
    scan: ScanInfo,
}

#[get("/scan/<id>")]
pub async fn rt(id: String) -> Json<Response<Data>> {
    let mut conn = establish_connection();

    let scan = match scan_info::table
        .filter(scan_info::id.eq(&id))
        .get_result::<ScanInfo>(&mut conn)
    {
        Ok(r) => r,
        Err(e) => {
            return Json(Response::error(ResError {
                msg: e.to_string(),
                detail: String::from("Failed to get scan info!"),
            }))
        }
    };

    Json(Response::data(Data { scan }))
}
