use diesel::prelude::*;
use diesel::backend::Backend;
use serde::Deserialize;
use serde::Serialize;
use crate::schema::posts;
use crate::schema::projects;
use super::queries::*;
use diesel::dsl::Filter;
use diesel::dsl::{AsSelect, Select};

#[derive(Queryable, PartialEq, Selectable, Debug)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub author: String,
    // pub creation_date: String,
    pub body: String,
    pub published: bool,
}

#[derive(Queryable, PartialEq, Selectable, Debug)]
pub struct Project {
    pub id: i32,
    pub title: String,
    pub description: String,
    // pub creation_date: String,
    // pub body: String,
    // pub published: bool,
}

#[derive(Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub author: &'a str,
    pub published: bool
}

#[derive(Insertable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = projects)]
pub struct NewProject {
    pub title: String,
    pub description: String
}
// pub struct NewProject<'a> {
//     pub id: i32,
//     pub title: &'a str,
//     pub description: &'a str
// }

pub struct PostQuery {
    pub title: Option<String>,
    pub author: Option<String>,
    pub id: Option<i32>,
}




