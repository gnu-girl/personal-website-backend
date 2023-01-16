/* File that holds all of te querying logic for finding blog entries */
use diesel::expression_methods::ExpressionMethods;
use diesel::dsl::Eq;
use diesel::prelude::sql_function;
use diesel::sql_types::{Text, Integer};
use super::super::schema::posts;
use crate::blog::models::Post;
use diesel::QueryDsl;

sql_function!(fn post_title(x: Text) -> Text);
sql_function!(fn post_id(x: Integer) -> Integer);
sql_function!(fn post_author(x: Text) -> Text);

/* Query fragment to make it easier to find posts by title */
pub type WithTitle<'a> =
    Eq<post_title::HelperType<posts::title>, post_title::HelperType<&'a str>>;
pub fn with_title(title: &str) -> WithTitle {
    post_title(posts::title).eq(post_title(title))
}

/* Query fragment to make it easier to find posts by id */
pub type WithId<'a> =
    Eq<post_id::HelperType<posts::id>, post_id::HelperType<&'a i32>>;
pub fn with_id(id: &i32) -> WithId {
    post_id(posts::id).eq(post_id(id))
}

/* Query fragment to make it easier to find posts by author */
type WithAuthor<'a> =
    Eq<post_author::HelperType<posts::author>, post_author::HelperType<&'a str>>;
fn with_author(author: &str) -> WithAuthor {
    post_author(posts::author).eq(post_author(author))
}

/// Shamelessly stolen from crates.io github
/// We literally never want to select `textsearchable_index_col`
/// so we provide this type and constant to pass to `.select`
type AllColumns = (
    posts::id,
    posts::title,
    posts::author,
    posts::body,
    posts::published,
);

pub const ALL_COLUMNS: AllColumns = (
    posts::id,
    posts::title,
    posts::author,
    posts::body,
    posts::published,
);

type All = diesel::dsl::Select<posts::table, AllColumns>;
type ByTitle<'a> = diesel::dsl::Filter<All, WithTitle<'a>>;
type ByAuthor<'a> = diesel::dsl::Filter<All, WithAuthor<'a>>;
type ById<'a> = diesel::dsl::Filter<All, WithId<'a>>;

/* TODO: Make these generic */
impl Post {
    pub fn with_title(title: &str) -> WithTitle {
        post_title(posts::title).eq(post_title(title))
    }

    pub fn by_title(title: &str) -> ByTitle {
        Post::all().filter(Self::with_title(title))
    }

    pub fn with_author(author: &str) -> WithAuthor {
        post_author(posts::author).eq(post_author(author))
    }

    pub fn by_author(author: &str) -> ByAuthor {
        Post::all().filter(Self::with_author(author))
    }

    pub fn with_id(id: &i32) -> WithId {
        post_id(posts::id).eq(post_id(id))
    }

    pub fn by_id(id: &i32) -> ById {
        Post::all().filter(Self::with_id(id))
    }

    pub fn all() -> All {
        posts::table.select(ALL_COLUMNS)
    }
}