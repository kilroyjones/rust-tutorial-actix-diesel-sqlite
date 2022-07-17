//! A simple Actix Web "Link saving service" to save and retrieve some links.
//!
//! Upon understanding the code, it is suggested to improve the code by adding
//! TODOs or solving them.
#[macro_use]
extern crate diesel;

mod models;
mod routes;
mod schema;

use actix_web::{web, App, HttpServer};
use diesel::r2d2::{self, ConnectionManager};
use diesel::sqlite::SqliteConnection;

/// Helps with changing the database engine without much edits.
pub type DatabaseConnection = SqliteConnection;
pub type Pool = r2d2::Pool<ConnectionManager<DatabaseConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    // Read the DATABASE_URL from the environment or set it to app.db assuming
    // Sqlite is used.
    // TODO one can improve this and set the `DatabaseConnection` to
    // PostgresConnection despite what user wants if the DATABASE_URL is not
    // set.
    let database_url = std::env::var("DATABASE_URL").unwrap_or("app.db".to_string());
    // To manage threads in an efficient way, we use database threadpools.
    let database_pool = Pool::builder()
        .build(ConnectionManager::<DatabaseConnection>::new(database_url))
        .unwrap();

    // TODO move the database_pool to a subsection of a custom AppData struct
    // that is derived from `Clone` and has such an impl:
    // impl AppState {
    //     fn new() -> web::Data<AppState> {
    //         web::Data<AppState> {
    //            ... // DATA here
    //         }
    //     }
    // }
    //
    // This is a more favorable approach, see: https://stackoverflow.com/a/65993435/8401058

    // TODO make this an TLS SSL Secure server and use `mkcert` to test it.
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(database_pool.clone())) // App data NEEDS to be wrapped in Data
            // TODO convert routes to services.
            // TODO implement authorization using JWT
            .route("/", web::get().to(routes::home))
            .route("/addlink", web::post().to(routes::add_link))
            .route("/getlinks", web::get().to(routes::get_links))
    })
    .bind("127.0.0.1:8888")?
    .run()
    .await
}
