use serde::{Deserialize, Serialize};

#[allow(non_camel_case_types)]
#[derive(Deserialize, Serialize)]
pub enum Response<T> {
    data(T),
    error { title: String, body: Option<String> },
}

pub mod scan;
pub mod stats;
pub mod tracks;
