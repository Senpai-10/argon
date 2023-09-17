mod run_scan;
mod scan_info;

pub fn routes() -> Vec<rocket::Route> {
    routes![run_scan::rt, scan_info::rt]
}
