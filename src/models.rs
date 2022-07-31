use crate::schema::*;
use serde::{Deserialize, Serialize};

// TODO implement From<> and Into<> traits for all of the three.
// TODO make a macro that automatically creates/guesses such structs.
// TODO move it all to `models/link.rs`

/// Represents a Link object from the database.
#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Link {
    pub id: i32,
    pub link: String,
    pub title: String,
    pub date_create: String,
}

/// Represents a link that is being newly generated.
#[derive(Debug, Insertable)]
#[table_name = "links"]
pub struct LinkNew<'a> {
    pub link: &'a str,
    pub title: &'a str,
    pub date_created: &'a str,
}

/// Represents a JSON data form filled request that will turn to a `Link`.
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkJson {
    pub link: String,
    pub title: String,
}
