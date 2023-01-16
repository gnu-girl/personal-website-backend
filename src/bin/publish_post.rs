use rust_zee::blog::models::Post;
use diesel::prelude::*;
use rust_zee::blog::client::establish_connection;
use std::env::args;

/* Update a draft post by changing a given post's field to published */
fn main() {
    use rust_zee::schema::posts::dsl::{posts, published};

    let id = args()
        .nth(1)
        .expect("publish_post requires a post id")
        .parse::<i32>()
        .expect("Invalid ID");
    let connection = &mut establish_connection();

    let post = diesel::update(posts.find(id))
        .set(published.eq(true))
        .get_result::<Post>(connection)
        .unwrap();
    println!("Published post {}", post.title);
}