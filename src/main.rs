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

    rocket::build().mount(
        "/api/",
        routes![
            routes::tracks::tracks,
            routes::tracks::track,
            routes::stream::stream,
            routes::picture::picture,
            routes::artists::artists,
            routes::artists::artist,
            routes::albums::albums,
            routes::albums::album,
            routes::stats::stats,
            routes::scan::scan_route,
            routes::search::search,
            routes::search::search_artists,
            routes::search::search_tracks,
            routes::search::search_albums,
            routes::favorites::favorites,
            routes::favorites::favorite_add,
            routes::favorites::favorite_remove,
            routes::auth::signup,
            routes::auth::login,
            routes::auth::logout,
            routes::playlists::playlists,
            routes::playlists::playlists_remove,
            routes::playlists::playlists_new,
            routes::playlists::playlists_new_track,
            routes::playlists::playlists_remove_track,
        ],
    )
}
