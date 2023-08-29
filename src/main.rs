#[macro_use]
extern crate rocket;

mod db;
mod models;
mod scan;
mod schema;

use scan::scan;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    env_logger::builder()
        .filter(None, log::LevelFilter::Info)
        .init();

    scan();

    rocket::build().mount("/", routes![index])
}
