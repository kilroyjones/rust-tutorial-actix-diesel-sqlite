#[macro_use]
extern crate diesel;

mod handlers;
mod schema;
mod models;

use actix_web::{web, App, HttpServer};
use anyhow::Result;
use diesel::r2d2::{self, ConnectionManager};
use diesel::{SqliteConnection};

pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[actix_rt::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug");
    
    let database_url = std::env::var("DATABASE_URL").expect("DB url not found");
    let database_pool = Pool::builder()
        .build(ConnectionManager::<SqliteConnection>::new(database_url))
        .unwrap();  

    HttpServer::new(move || {
        App::new()
            .data(database_pool.clone())
            .route("/", web::get().to(handlers::home))
            .route("/addlink", web::post().to(handlers::add_link))
            .route("/getlinks", web::get().to(handlers::get_links))
        })

    .bind("127.0.0.1:8888")?
    .run()
    .await?;

    Ok(())
}