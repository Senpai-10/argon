use crate::schema;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = schema::scan_info)]
pub struct ScanInfo {
    pub id: i32,
    pub scan_start: NaiveDateTime,
    pub scan_end: NaiveDateTime,
    pub artists: i32,
    pub albums: i32,
    pub tracks: i32,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = schema::scan_info)]
pub struct NewScanInfo {
    pub scan_start: NaiveDateTime,
    pub scan_end: NaiveDateTime,
    pub artists: i32,
    pub albums: i32,
    pub tracks: i32,
}
