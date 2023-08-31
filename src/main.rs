#[macro_use]
extern crate rocket;

mod db;
mod models;
mod routes;
mod scan;
mod schema;

use dotenvy::dotenv;
use scan::{is_first_run, scan};

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    env_logger::builder()
        .filter(None, log::LevelFilter::Info)
        .init();

    {
        let mut conn = db::establish_connection();

        if is_first_run(&mut conn) {
            scan(&mut conn);
        }
    }

    rocket::build().mount(
        "/api/",
        routes![routes::tracks::tracks, routes::tracks::tracks_feed],
    )
}
