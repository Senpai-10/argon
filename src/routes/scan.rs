use crate::db;
use crate::models::scan_info::ScanInfo;
use crate::schema;
use diesel::prelude::*;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

use crate::scan::scan;
use dirs::home_dir;
use std::path::PathBuf;

#[allow(non_camel_case_types)]
#[derive(Deserialize, Serialize)]
pub enum Response<T> {
    data(T),
    error { msg: String },
}

#[derive(Deserialize, Serialize)]
pub struct Data {
    scan_info: Option<ScanInfo>,
}

#[get("/scan?<force>")]
pub async fn scan_route(force: Option<bool>) -> Json<Response<Data>> {
    if scan_lock_file_path().exists() {
        return Json(Response::error {
            msg: "Already scanning".into(),
        });
    }

    let mut conn = db::establish_connection();

    if force.is_some() {
        diesel::delete(schema::albums::dsl::albums).execute(&mut conn);
        diesel::delete(schema::tracks::dsl::tracks).execute(&mut conn);
        diesel::delete(schema::artists::dsl::artists).execute(&mut conn);
    }

    scan_lock();

    let scan_info = scan(&mut conn);

    scan_unlock();

    Json(Response::data(Data { scan_info }))
}

fn scan_lock_file_path() -> PathBuf {
    home_dir().unwrap().join(".argon-scan-lock")
}

fn scan_lock() {
    let file = scan_lock_file_path();

    if !file.exists() {
        _ = std::fs::write(file, "");
    }
}

fn scan_unlock() {
    let file = scan_lock_file_path();

    if file.exists() {
        _ = std::fs::remove_file(file);
    }
}
