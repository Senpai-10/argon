use crate::schema;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize, Clone)]
#[diesel(table_name = schema::scan_info)]
pub struct ScanInfo {
    pub id: String,
    pub scan_start: NaiveDateTime,
    pub scan_end: Option<NaiveDateTime>,
    pub is_done: bool,
    pub artists: i32,
    pub albums: i32,
    pub tracks: i32,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = schema::scan_info)]
pub struct NewScanInfo {
    pub id: String,
    pub scan_start: NaiveDateTime,
    pub scan_end: Option<NaiveDateTime>,
    pub is_done: bool,
    pub artists: i32,
    pub albums: i32,
    pub tracks: i32,
}
