use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use crate::{blog::models::{NewPost, Post, PostQuery}, schema::posts};
use crate::blog::queries::*;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_post(conn: &mut PgConnection, title: &str, body: &str, author: &str, published: bool) -> Post {

    let new_post = NewPost { title, body, author, published };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn publish_draft_post(conn: &mut PgConnection, query_id: i32) -> Post {
    use crate::schema::posts::dsl::*;

    diesel::update(posts.find(query_id))
        .set(published.eq(true))
        .get_result::<Post>(conn)
        .unwrap()
}

pub fn delete_post(conn: &mut PgConnection, query_id: i32) -> usize {
    use crate::schema::posts::dsl::*;
    
    diesel::delete(posts.filter(id.eq(query_id)))
        .execute(conn)
        .expect("Error deleting posts")
}

// Return published posts matching id
pub fn find_post(conn: &mut PgConnection, query_id: i32) -> Vec<Post> {
    use crate::schema::posts::dsl::*;
    
    //TODO: Actually use the queries here
    // let results = Post
    //     ::by_id(&id)
    //     .load::<Post>(conn)
    //     .expect(format!("Can't find post with given id {:?}", id).as_str());

    let results = posts
        .filter(published.eq(true))
        .filter(id.eq(query_id))
        .limit(1)
        .load::<Post>(conn)
        .expect("Error loading posts");
    results
}