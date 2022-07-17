//! This file holds all the routes and request related functions.
// TODO rename to handlers as it is more common in Actix Web and also handlers
// files are expected to host services alongside routes but routes files are not
// expected to have handlers in them.
use crate::models::{Link, LinkJson, LinkNew};
use crate::Pool;

use actix_web::http::StatusCode;
use actix_web::{web, Error, HttpResponse};
use anyhow::Result;
use diesel::dsl::insert_into;
use diesel::prelude::*;
use diesel::RunQueryDsl;

/// Returns the home page of the site.
pub async fn home() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../templates/index.html")))
}

/// Adds a link entry to the database if not already added.
pub async fn add_link(
    pool: web::Data<Pool>,
    item: web::Json<LinkJson>,
) -> Result<HttpResponse, Error> {
    Ok(
        match web::block(move || add_single_link(pool, item)).await {
            Ok(link) => HttpResponse::Created().json(link),
            _ => HttpResponse::from(HttpResponse::InternalServerError()),
        },
    )
}

/// Get all the links in the database.
pub async fn get_links(pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(match get_all_links(pool).await {
        Ok(links) => HttpResponse::Ok().json(links),
        _ => HttpResponse::from(HttpResponse::InternalServerError()),
    })
}

// TODO move to a method of the `Link`.
fn add_single_link(pool: web::Data<Pool>, item: web::Json<LinkJson>) -> Link {
    use crate::schema::links::dsl::*;
    let db_connection = pool.get().unwrap();

    match links
        .filter(link.eq(&item.link))
        .first::<Link>(&db_connection)
    {
        Ok(result) => result,
        Err(_) => {
            let new_link = LinkNew {
                link: &item.link,
                title: &item.title,
                date_created: &format!("{}", chrono::Local::now().naive_local()),
            };

            insert_into(links)
                .values(&new_link)
                .execute(&db_connection)
                .expect("Error saving new link");

            let result = links.order(id.desc()).first(&db_connection).unwrap();
            result
        }
    }
}

// TODO move to a method of the `Link`.
async fn get_all_links(pool: web::Data<Pool>) -> Result<Vec<Link>, diesel::result::Error> {
    use crate::schema::links::dsl::*;
    let db_connection = pool.get().unwrap();
    let result = links.load::<Link>(&db_connection)?;
    Ok(result)
}
