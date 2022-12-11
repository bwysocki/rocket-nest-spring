use crate::schema::posts;
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use std::cmp::{Ord, Eq, PartialOrd, PartialEq};

#[derive(QueryableByName, Queryable, Serialize, Ord, Eq, PartialEq, PartialOrd, Debug)]
#[diesel(table_name = posts)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub genre: String,
    pub published: bool,
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = posts)]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub genre: &'a str,
}