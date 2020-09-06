use serde::{Deserialize, Serialize};
use crate::schema::*;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Link {
    pub id: i32,
    pub link: String,
    pub title: String,
    pub date_create: String
}

#[derive(Debug, Insertable)]
#[table_name = "links"]
pub struct LinkNew<'a> {
    pub link: &'a str,
    pub title: &'a str,
    pub date_created: &'a str
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LinkJson {
    pub link: String,
    pub title: String
}