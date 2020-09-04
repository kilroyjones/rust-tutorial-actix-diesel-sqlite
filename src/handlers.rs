use crate::models::{Link, LinkNew, LinkJson};
use crate::Pool;

use actix_web::{web, HttpResponse, Error};
use actix_web::http::{StatusCode};
use diesel::RunQueryDsl;
use diesel::dsl::{insert_into};
use diesel::prelude::*;
use anyhow::Result;
use chrono;

pub async fn home() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../templates/index.html")))
}


pub async fn add_link(
    pool: web::Data<Pool>,
    item: web::Json<LinkJson>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || add_single_link(pool, item))
            .await
            .map(|link| HttpResponse::Created().json(link))
            .map_err(|_| HttpResponse::InternalServerError())?
    )
}


fn add_single_link(
    pool: web::Data<Pool>,
    item: web::Json<LinkJson>,
) -> Result<Link, diesel::result::Error> {
    use crate::schema::links::dsl::*;
    let db_connection = pool.get().unwrap();

    match links
        .filter(link.eq(&item.link))
        .first::<Link>(&db_connection) {
            Ok(res) => Ok(res),
            Err(_) => {
                let new_link = LinkNew {
                    link: &item.link,
                    title: &item.title,
                    date_created: &format!("{}", chrono::Local::now()
                        .naive_local())
                };

                insert_into(links)
                    .values(&new_link)
                    .execute(&db_connection)
                    .expect("Error saving new post");
                
                let result = links.order(id.desc())
                    .first(&db_connection).unwrap();

                Ok(result)
            }
        }
}

pub async fn get_links(
    pool: web::Data<Pool>)
-> Result<HttpResponse, Error> {
    Ok(get_all_links(pool)
        .await
        .map(|links| HttpResponse::Ok().json(links))
        .map_err(|_| HttpResponse::InternalServerError())?
    )
}

async fn get_all_links(
    pool: web::Data<Pool>
) -> Result<Vec<Link>, diesel::result::Error> {
    use crate::schema::links::dsl::*;
    let conn = pool.get().unwrap();
    let result = links.load::<Link>(&conn)?;
    Ok(result)
}

