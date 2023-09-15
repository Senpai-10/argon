#[macro_use]
extern crate rocket;

mod auth;
mod db;
mod models;
mod routes;
mod scan;
mod schema;

use dotenvy::dotenv;
use scan::{is_first_run, Scanner};

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    env_logger::builder()
        .filter(None, log::LevelFilter::Info)
        .init();

    {
        let mut conn = db::establish_connection();

        if is_first_run(&mut conn) {
            let mut scanner = Scanner::new(conn);

            if let Err(e) = scanner.start() {
                error!("Scanner error: {}", e.to_string())
            }
        }
    }

    rocket::build().mount("/api/", routes::routes())
}
