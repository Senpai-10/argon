#[macro_use]
extern crate rocket;

mod db;
mod models;
mod schema;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    env_logger::builder()
        .filter(None, log::LevelFilter::Info)
        .init();

    rocket::build().mount("/", routes![index])
}
