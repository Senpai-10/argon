#[macro_use]
extern crate rocket;

mod auth;
mod db;
mod models;
mod routes;
mod scanner;
mod schema;

use dotenvy::dotenv;
use scanner::Scanner;

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    env_logger::builder()
        .filter(None, log::LevelFilter::Info)
        .init();

    {
        use diesel::prelude::*;
        use schema::scan_info;

        let mut conn = db::establish_connection();

        let count = scan_info::table
            .count()
            .get_result::<i64>(&mut conn)
            .unwrap_or(0);

        if count == 0 {
            let mut scanner = Scanner::new(conn);

            if scanner.is_locked() {
                error!("Scanner is already running");
            } else {
                scanner.start()
            }
        }
    }

    rocket::build().mount("/api/", routes::routes())
}
