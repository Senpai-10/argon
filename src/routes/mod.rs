use serde::{Deserialize, Serialize};

#[allow(non_camel_case_types)]
#[derive(Deserialize, Serialize)]
pub enum Response<T> {
    data(T),
    error { msg: String },
}

pub mod albums;
pub mod artists;
pub mod scan;
pub mod search;
pub mod stats;
pub mod stream;
pub mod tracks;
