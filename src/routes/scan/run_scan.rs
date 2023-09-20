use crate::routes::prelude::*;
use crate::scanner::{lock_manager, Scanner};

#[derive(Deserialize, Serialize)]
pub struct Data {
    scan_id: String,
}

#[post("/scan")]
pub async fn rt() -> Json<Response<Data>> {
    let conn = establish_connection();

    let mut scanner = Scanner::new(conn);

    if lock_manager::is_locked() {
        return Json(Response::error(ResError {
            msg: "Scanner is already running".to_string(),
            detail: String::from(""),
        }));
    }

    let data = Data {
        scan_id: scanner.id.clone(),
    };

    // Run scanner in another thread
    //      And return the scan id
    //      So we don't timeout the connection
    std::thread::spawn(move || scanner.start());

    Json(Response::data(data))
}
